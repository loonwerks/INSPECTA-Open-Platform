#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include <os/sddf.h>
#include <sddf/network/config.h>
#include <sddf/network/queue.h>
#include <sddf/util/util.h>

/*
 * QEMU backend for the HAMR low-level Ethernet component.
 *
 * sdfgen fills this section with the queues and data regions belonging to the
 * component's sDDF network client.  The Rust application calls the small API
 * below so its board-independent HAMR port handling remains unchanged.
 */
__attribute__((__section__(".net_client_config"))) net_client_config_t net_config;

static net_queue_handle_t rx_queue;
static net_queue_handle_t tx_queue;

void qemu_virtio_net_init(void)
{
    assert(net_config_check_magic(&net_config));

    net_queue_init(&rx_queue, net_config.rx.free_queue.vaddr,
                   net_config.rx.active_queue.vaddr,
                   net_config.rx.num_buffers);
    net_queue_init(&tx_queue, net_config.tx.free_queue.vaddr,
                   net_config.tx.active_queue.vaddr,
                   net_config.tx.num_buffers);

    /* The client owns the Tx buffers. The Rx copier owns the Rx buffers. */
    net_buffers_init(&tx_queue, 0);
}

size_t qemu_virtio_net_receive(uint8_t *destination, size_t capacity)
{
    net_buff_desc_t buffer;
    if (net_dequeue_active(&rx_queue, &buffer) != 0) {
        return 0;
    }

    size_t length = buffer.len;
    if (length <= capacity &&
        buffer.io_or_offset < net_config.rx_data.size &&
        length <= net_config.rx_data.size - buffer.io_or_offset) {
        memcpy(destination,
               (uint8_t *)net_config.rx_data.vaddr + buffer.io_or_offset,
               length);
    } else {
        length = 0;
    }

    buffer.len = 0;
    assert(net_enqueue_free(&rx_queue, buffer) == 0);
    if (net_require_signal_free(&rx_queue)) {
        net_cancel_signal_free(&rx_queue);
        sddf_notify(net_config.rx.id);
    }

    return length;
}

bool qemu_virtio_net_send(const uint8_t *source, size_t length)
{
    if (length == 0 || length > NET_BUFFER_SIZE) {
        return false;
    }

    net_buff_desc_t buffer;
    if (net_dequeue_free(&tx_queue, &buffer) != 0) {
        return false;
    }

    if (buffer.io_or_offset >= net_config.tx_data.size ||
        length > net_config.tx_data.size - buffer.io_or_offset) {
        buffer.len = 0;
        assert(net_enqueue_free(&tx_queue, buffer) == 0);
        return false;
    }

    memcpy((uint8_t *)net_config.tx_data.vaddr + buffer.io_or_offset,
           source, length);
    buffer.len = length;
    assert(net_enqueue_active(&tx_queue, buffer) == 0);

    if (net_require_signal_active(&tx_queue)) {
        net_cancel_signal_active(&tx_queue);
        sddf_notify(net_config.tx.id);
    }

    return true;
}
