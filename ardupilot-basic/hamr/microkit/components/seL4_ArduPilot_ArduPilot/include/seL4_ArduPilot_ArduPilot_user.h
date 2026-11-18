/*
 * Copyright 2024, DornerWorks
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */
#pragma once

#include <microkit.h>

// This file will not be overwritten if HAMR codegen is rerun

#if defined(BOARD_qemu_virt_aarch64)
#define GUEST_RAM_SIZE            0x3f000000
#define GUEST_DTB_VADDR             0xbef00000
#define GUEST_INIT_RAM_DISK_VADDR   0xa0000000
#elif defined(BOARD_zcu102)
#define GUEST_RAM_SIZE            0x40000000
#define GUEST_DTB_VADDR             0x820000000
#define GUEST_INIT_RAM_DISK_VADDR   0x820100000
#else
#error Need to define guest kernel image address and DTB address
#endif

#if defined(BOARD_zcu102) || defined(BOARD_qemu_virt_aarch64)
#define MAX_IRQS 2
#else
#define MAX_IRQS 1
#endif

#if defined(BOARD_qemu_virt_aarch64)
#define SERIAL_IRQ_CH 1
#define SERIAL_IRQ 33
#define VIRTIO_BLK_IRQ_CH 2
#define VIRTIO_BLK_IRQ 56
#elif defined(BOARD_zcu102)
#define SERIAL_IRQ_CH 1
#define SERIAL_IRQ 53
#define MMC_IRQ_CH 2
#define MMC_IRQ 81
#else
#error Need to define IRQs
#endif

// Device IRQs passed through to the guest: the guest sees interrupt 'irq',
// and this component is notified on 'channel' when it fires.
struct mk_irq {
  int irq;
  microkit_channel channel;
};

struct mk_irq mk_irqs[MAX_IRQS] = {
  { // Serial
    .irq = SERIAL_IRQ,
    .channel = SERIAL_IRQ_CH
  }
#if defined(BOARD_zcu102)
  , { 
     .irq = MMC_IRQ, 
     .channel = MMC_IRQ_CH 
  }
#elif defined(BOARD_qemu_virt_aarch64)
  , {
     .irq = VIRTIO_BLK_IRQ,
     .channel = VIRTIO_BLK_IRQ_CH
  }
#endif
};
