/*
 * Copyright 2024, UNSW (ABN 57 195 873 179)
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */
#include <libvmm/virq.h>
#include <libvmm/virtio/config.h>
#include <libvmm/virtio/virtio.h>
#include <libvmm/virtio/virtq.h>
#include <libvmm/virtio/mmio.h>
#include "net.h"
#include <libvmm/util/util.h>
// #include <sddf/network/queue.h>

#ifndef BIT_LOW
#define BIT_LOW(n) (1U << (n))
#endif

/* Uncomment this to enable debug logging */
#define DEBUG_NET

#if defined(DEBUG_NET)
#define LOG_NET(...) do{ printf("VIRTIO(NET): "); printf(__VA_ARGS__); }while(0)
#else
#define LOG_NET(...) do{}while(0)
#endif

#define LOG_NET_ERR(...) do{ printf("VIRTIO(NET)|ERROR: "); printf(__VA_ARGS__); }while(0)


static inline struct virtio_net_device *device_state(struct virtio_device *dev)
{
    return (struct virtio_net_device *)dev->device_data;
}

static void virtio_net_reset(struct virtio_device *dev)
{
    LOG_NET("operation: reset\n");
    for (int i = 0; i < dev->num_vqs; i++) {
        dev->vqs[i].ready = false;
        dev->vqs[i].last_idx = 0;
    }
}

static bool driver_ok(struct virtio_device *dev)
{
    return (dev->regs.Status & VIRTIO_CONFIG_S_DRIVER_OK) &&
           (dev->regs.Status & VIRTIO_CONFIG_S_FEATURES_OK);
}

static bool virtio_net_get_device_features(struct virtio_device *dev, uint32_t *features)
{
    LOG_NET("operation: get device features\n");

    if (dev->regs.Status & VIRTIO_CONFIG_S_FEATURES_OK) {
        LOG_NET_ERR("Driver tried to read device features after FEATURES_OK\n");
    }

    switch (dev->regs.DeviceFeaturesSel) {
    /* Feature bits 0 to 31 */
    case 0:
        *features = BIT_LOW(VIRTIO_NET_F_MAC) | BIT_LOW(VIRTIO_NET_F_MTU);
        break;
    /* Features bits 32 to 63 */
    case 1:
        *features = BIT_HIGH(VIRTIO_F_VERSION_1);
        break;
    default:
        LOG_NET_ERR("Bad DeviceFeaturesSel 0x%x\n", dev->regs.DeviceFeaturesSel);
        return false;
    }
    return true;
}


static bool virtio_net_set_driver_features(struct virtio_device *dev, uint32_t features)
{
    bool success = true;
    LOG_NET("data.DriverFeaturesSel: %u\n", dev->regs.DriverFeaturesSel);
    LOG_NET("features: 0x%x\n", features);

    switch (dev->regs.DriverFeaturesSel) {
    /* Feature bits 0 to 31 */
    case 0:
        /** F_MAC is required */
        // success = (features == (BIT_LOW(VIRTIO_NET_F_MAC) | BIT_LOW(VIRTIO_NET_F_MTU)));
        success = (features == BIT_LOW(VIRTIO_NET_F_MAC));
        break;

    /* Features bits 32 to 63 */
    case 1:
        success = (features == BIT_HIGH(VIRTIO_F_VERSION_1));
        break;

    default:
        LOG_NET_ERR("Bad DriverFeaturesSel 0x%x\n", dev->regs.DriverFeaturesSel);
        success = false;
    }
    if (success) {
        dev->features_happy = 1;
    }
    return success;
}

static bool virtio_net_get_device_config(struct virtio_device *dev,
                                         uint32_t offset,
                                         uint32_t *ret_val)
{
    struct virtio_net_config *config = &device_state(dev)->config;

    uint32_t word_offset = offset / sizeof(uint32_t);
    LOG_NET("device config: %u\n", word_offset);
    switch (word_offset) {
    case 0:
        *ret_val = config->mac[0];
        *ret_val |= config->mac[1] << 8;
        *ret_val |= config->mac[2] << 16;
        *ret_val |= config->mac[3] << 24;
        break;

    case 1:
        *ret_val = config->mac[4];
        *ret_val |= config->mac[5] << 8;
        break;

    case 2:
        *ret_val = (uint32_t)config->mtu;
        break;

    default:
        LOG_NET_ERR("Unknown device config register: 0x%x\n", offset);
        return false;
    }
    return true;
}

static bool virtio_net_set_device_config(struct virtio_device *dev, uint32_t offset, uint32_t val)
{
    LOG_NET_ERR("All configuration fields are readonly\n");
    return false;
}

static bool virtio_net_respond(struct virtio_device *dev)
{
#ifdef ARM_GIC_IRQ_ROUTE
    virtio_set_interrupt_status(dev, true, false);
    bool success = virtio_inject_interrupt(dev);
#else
    dev->regs.InterruptStatus = BIT_LOW(0);
    bool success = virq_inject(dev->virq);
#endif
    assert(success);

    return success;
}

/* HAMR's fixed-size Ethernet port contains one zero-padded frame. Assemble
 * chained guest descriptors into that frame before invoking the application. */
static bool virtio_net_queue_notify(struct virtio_device *dev)
{
    if (!driver_ok(dev)) return false;
    if (dev->regs.QueueSel == VIRTIO_NET_RX_VIRTQ) return true;
    if (dev->regs.QueueSel != VIRTIO_NET_TX_VIRTQ) return false;
    virtio_queue_handler_t *vq = &dev->vqs[VIRTIO_NET_TX_VIRTQ];
    if (!vq->ready) return false;

    uint16_t head;
    bool completed = false;
    /* Bound the work per notification even if a guest keeps adding buffers. */
    for (unsigned n = 0; n < vq->virtq.num && virtio_virtq_pop_avail(vq, &head); n++) {
        uint8_t frame[1600] = {0};
        const size_t header = sizeof(struct virtio_net_hdr_mrg_rxbuf);
        uint64_t total = virtio_desc_chain_payload_len(vq, head);
        if (total > header && total - header <= sizeof(frame) &&
            virtio_read_data_from_desc_chain(vq, head, total - header, header, (char *)frame)) {
            vmm_virtio_net_tx(frame);
        }
        /* TX buffers are read-only: report zero bytes written to the guest. */
        virtio_virtq_add_used(vq, head, 0);
        completed = true;
    }
    return !completed || virtio_net_respond(dev);
}

static void handle_rx_buffer(struct virtio_device *dev, void *buf, uint32_t size,
                             bool *respond_to_guest)
{
    virtio_queue_handler_t *vq = &dev->vqs[VIRTIO_NET_RX_VIRTQ];
    uint16_t head;
    if (!virtio_virtq_pop_avail(vq, &head)) return;
    struct virtio_net_hdr_mrg_rxbuf header = {0};
    header.num_buffers = 1;
    bool success = virtio_write_data_to_desc_chain(vq, head, sizeof(header), 0, (char *)&header)
        && virtio_write_data_to_desc_chain(vq, head, size, sizeof(header), (char *)buf);
    virtio_virtq_add_used(vq, head, success ? sizeof(header) + size : 0);
    *respond_to_guest = true;
}

bool custom_virtio_net_handle_rx(struct virtio_net_device *state, void *buf, uint32_t size)
{
    struct virtio_device *dev = &state->virtio_device;

    if (!driver_ok(dev)) {
        return false;
    }
    if (!dev->vqs[VIRTIO_NET_RX_VIRTQ].ready) {
        /* vq is not initialised, drop the packet */
        return false;
    }

    // net_buff_desc_t sddf_buffer;
    // bool reprocess = true;
    bool respond_to_guest = false;
    
    handle_rx_buffer(dev, buf, size, &respond_to_guest);


    // if (respond_to_guest) {
    //     return virtio_net_respond(dev);
    // }

    return respond_to_guest;
}

void custom_virtio_net_respond_to_guest(struct virtio_net_device *state) {
    struct virtio_device *dev = &state->virtio_device;
    virtio_net_respond(dev);
}

static virtio_device_funs_t functions = {
    .device_reset = virtio_net_reset,
    .get_device_features = virtio_net_get_device_features,
    .set_driver_features = virtio_net_set_driver_features,
    .get_device_config = virtio_net_get_device_config,
    .set_device_config = virtio_net_set_device_config,
    .queue_notify = virtio_net_queue_notify,
};

bool custom_virtio_mmio_net_init(struct virtio_net_device *net_dev,
                          uint8_t mac[VIRTIO_NET_CONFIG_MAC_SZ],
                          uint16_t mtu,
                          uintptr_t region_base,
                          uintptr_t region_size,
                          size_t virq)
{
    struct virtio_device *dev = &net_dev->virtio_device;

    dev->regs.DeviceID = VIRTIO_DEVICE_ID_NET;
    dev->regs.VendorID = VIRTIO_DEV_VENDOR_ID;
    dev->transport_type = VIRTIO_TRANSPORT_MMIO;
    dev->funs = &functions;
    dev->vqs = net_dev->vqs;
    dev->num_vqs = VIRTIO_NET_NUM_VIRTQ;
#ifdef ARM_GIC_IRQ_ROUTE
    dev->irq_routing_info = ARM_GIC_IRQ_ROUTE(0, virq);
#else
    dev->virq = virq;
#endif
    dev->device_data = net_dev;

    memcpy(net_dev->config.mac, mac, VIRTIO_NET_CONFIG_MAC_SZ);
    net_dev->config.mtu = mtu;

#ifdef ARM_GIC_IRQ_ROUTE
    return virtio_mmio_register_device(dev, region_base, region_size, dev->irq_routing_info);
#else
    return virtio_mmio_register_device(dev, region_base, region_size, virq);
#endif
}
