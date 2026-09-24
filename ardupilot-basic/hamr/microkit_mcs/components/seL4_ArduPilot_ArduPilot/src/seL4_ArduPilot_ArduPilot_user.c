#include <seL4_ArduPilot_ArduPilot.h>
#include <seL4_ArduPilot_ArduPilot_user.h>
#include <libvmm/guest.h>
#include <libvmm/virq.h>
#include <libvmm/util/util.h>
#include <libvmm/arch/aarch64/linux.h>
#include <libvmm/arch/aarch64/fault.h>

#include "virtio/net.h"
static struct virtio_net_device virtio_net;
#if defined(BOARD_zcu102)
#include <libvmm/arch/aarch64/smc.h>
#endif

// This file will not be overwritten if HAMR codegen is rerun

// Data for the guest's kernel image.
extern char _guest_kernel_image[];
extern char _guest_kernel_image_end[];

// Data for the device tree to be passed to the kernel.
extern char _guest_dtb_image[];
extern char _guest_dtb_image_end[];

// Data for the initial RAM disk to be passed to the kernel.
extern char _guest_initrd_image[];
extern char _guest_initrd_image_end[];

// Microkit will set this variable to the start of the guest RAM memory region.
uintptr_t ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM_vaddr;

static bool is_passthrough_irq_ch(microkit_channel ch);

void seL4_ArduPilot_ArduPilot_initialize(void) {
  // Initialise the VMM, the VCPU(s), and start the guest
  LOG_VMM("starting \"%s\"\n", microkit_name);

  // As in microkit/vmm/vmm.c, use the RAM address supplied by Microkit.
  // meta.py maps guest RAM at the same address in the VMM and the guest.
  uintptr_t guest_ram_vaddr = ZCU102_Impl_Instance_seL4_ArduPilot_ArduPilot_VM_Guest_RAM_vaddr;

  // The LionsOS libvmm in jasonbelt/microkit_provers requires RAM registration
  // before linux_setup_images translates guest physical addresses for copying.
  arch_guest_init_t args = {
    .pci_init.mmio_aperature_size = 0, // no virtual PCI bus
    .num_vcpus = 1,
    .num_guest_ram_regions = 1,
    .guest_ram_regions = {{
      .gpa_start = guest_ram_vaddr,
      .size = GUEST_RAM_SIZE,
      .vmm_vaddr = (void *) guest_ram_vaddr,
    }},
  };
  bool success = guest_init(args);
  if (!success) {
    LOG_VMM_ERR("Failed to initialise guest\n");
    return;
  }

  // Place all the binaries in the right locations before starting the guest

  size_t kernel_size = _guest_kernel_image_end - _guest_kernel_image;
  size_t dtb_size = _guest_dtb_image_end - _guest_dtb_image;
  size_t initrd_size = _guest_initrd_image_end - _guest_initrd_image;

  uintptr_t kernel_pc = linux_setup_images(guest_ram_vaddr,
                                          (uintptr_t) _guest_kernel_image,
                                          kernel_size,
                                          (uintptr_t) _guest_dtb_image,
                                          GUEST_DTB_VADDR,
                                          dtb_size,
                                          (uintptr_t) _guest_initrd_image,
                                          GUEST_INIT_RAM_DISK_VADDR,
                                          initrd_size);

  if (!kernel_pc) {
    LOG_VMM_ERR("Failed to initialise guest images\n");
    return;
  }

  // Register the pass-through device IRQs. libvmm acks the hardware IRQ itself
  // once the guest acks the virtual one, so no ack handler is needed here.
  for(int i=0; i < MAX_IRQS; i++) {
#ifdef ARM_GIC_IRQ_ROUTE
    success = virq_register_passthrough(ARM_GIC_IRQ_ROUTE(GUEST_BOOT_VCPU_ID, mk_irqs[i].irq), mk_irqs[i].channel);
#else
    success = virq_register_passthrough(GUEST_BOOT_VCPU_ID, mk_irqs[i].irq, mk_irqs[i].channel);
#endif
    if (!success) {
      LOG_VMM_ERR("Failed to register pass-through IRQ %d\n", mk_irqs[i].irq);
      return;
    }
    // Just in case there are already interrupts available to handle, we ack them here.
    microkit_irq_ack(mk_irqs[i].channel);
  }

#if defined(BOARD_zcu102)
  if (!smc_register_sip_handler(smc_sip_forward)) {
    LOG_VMM_ERR("Failed to initialise SMC SIP handler\n");
    return;
  }
#endif

  uint8_t mac[VIRTIO_NET_CONFIG_MAC_SZ] = {0x00, 0x0a, 0x35, 0x03, 0x78, 0xa1};
  if (!custom_virtio_mmio_net_init(&virtio_net, mac, 1600, 0x150000, 0x1000, 129)) {
    LOG_VMM_ERR("Failed to initialise virtio network\n");
    return;
  }

  // Finally start the guest
  success = guest_start(kernel_pc, GUEST_DTB_VADDR, GUEST_INIT_RAM_DISK_VADDR);
  if (!success) {
    LOG_VMM_ERR("Failed to start guest\n");
    return;
  }

  LOG_VMM("Guest started, leaving seL4_ArduPilot_ArduPilot_initialize\n");
}

static uint8_t tx_idx = 0;

void vmm_virtio_net_tx(void *tx_buf) {
    // LOG_VMM("Sending TX Message from guest\n");
    switch (tx_idx) {
        case 0:
            put_EthernetFramesTx0((SW_RawEthernetMessage *)tx_buf);
            break;
        case 1:
            put_EthernetFramesTx1((SW_RawEthernetMessage *)tx_buf);
            break;
        case 2:
            put_EthernetFramesTx2((SW_RawEthernetMessage *)tx_buf);
            break;
        case 3:
            put_EthernetFramesTx3((SW_RawEthernetMessage *)tx_buf);
            break;
    }
    tx_idx = (tx_idx + 1) % 4;
    // LOG_VMM("TX Packet: ");
    // int i;
    // uint8_t* tx = tx_buf;

    // for(i=0; i<128; i++) {
    //     printf("%02x ", tx[i]);
    // }
    // printf("\n");
}

bool get_FirewallRx(uint8_t idx, SW_RawEthernetMessage *data) {
    bool avail = false;
    switch (idx) {
        case 0:
            avail = !FirewallRx0_is_empty();
            if (avail) {
                get_FirewallRx0(data);
            }
            return avail;
        case 1:
            avail = !MavlinkRx0_is_empty();
            if (avail) {
                get_MavlinkRx0(data);
            }
            return avail;
        case 2:
            avail = !FirewallRx1_is_empty();
            if (avail) {
                get_FirewallRx1(data);
            }
            return avail;
        case 3:
            avail = !MavlinkRx1_is_empty();
            if (avail) {
                get_MavlinkRx1(data);
            }
            return avail;
        case 4:
            avail = !FirewallRx2_is_empty();
            if (avail) {
                get_FirewallRx2(data);
            }
            return avail;
        case 5:
            avail = !MavlinkRx2_is_empty();
            if (avail) {
                get_MavlinkRx2(data);
            }
            return avail;
        case 6:
            avail = !FirewallRx3_is_empty();
            if (avail) {
                get_FirewallRx3(data);
            }
            return avail;
        case 7:
            avail = !MavlinkRx3_is_empty();
            if (avail) {
                get_MavlinkRx3(data);
            }
            return avail;
        default:
            return false;
    }
}


void seL4_ArduPilot_ArduPilot_timeTriggered(void) {
    // printf("Ardupilot: Time Triggered\n");
    // TODO: Implement API funcs <-> virtio-net backend translation
    SW_RawEthernetMessage rx;
    for(int i = 0; i < 8; i++){
        if (get_FirewallRx(i, &rx)) {
            bool respond = custom_virtio_net_handle_rx(&virtio_net, &rx, 1600);
            if (respond) {
                 custom_virtio_net_respond_to_guest(&virtio_net);
            }
            // int i;
            // LOG_VMM("Ardu: Rx Packet: ");

            // for(i=0; i<128; i++) {
            //     printf("%02x ", rx[i]);
            // }
            // printf("\n");
        }
    }
}

void seL4_ArduPilot_ArduPilot_notify(microkit_channel ch) {
  if (is_passthrough_irq_ch(ch)) {
    if (!virq_handle_passthrough(ch)) {
      LOG_VMM_ERR("IRQ dropped on channel 0x%x\n", ch);
    }
    return;
  }

  printf("Unexpected channel, ch: 0x%x\n", ch);
}

/*
 * The primary purpose of the VMM after initialisation is to act as a fault-handler.
 * Whenever our guest causes an exception, it gets delivered to this entry point for
 * the VMM to handle.
 */
seL4_Bool fault(microkit_child child, microkit_msginfo msginfo, microkit_msginfo *reply_msginfo) {
    bool success = fault_handle(child, msginfo);
    if (success) {
        // Now that we have handled the fault successfully, we reply to it so
        // that the guest can resume execution.
        *reply_msginfo = microkit_msginfo_new(0, 0);
        return seL4_True;
    }

    return seL4_False;
}

static bool is_passthrough_irq_ch(microkit_channel ch) {
  for(int i=0; i < MAX_IRQS; i++) {
    if (mk_irqs[i].channel == ch) {
      return true;
    }
  }

  return false;
}
