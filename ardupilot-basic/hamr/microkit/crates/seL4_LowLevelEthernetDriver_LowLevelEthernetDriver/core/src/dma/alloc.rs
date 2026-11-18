//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

use core::ptr::NonNull;
use log::debug;

pub struct GemDmaPtrs {
    pub rx: DmaPtrs,
    pub tx: DmaPtrs,
    pub tx_dummy: DmaPtr,
}

pub struct DmaPtrs {
    pub desc: DmaPtr,
    pub buf: DmaPtr,
}

pub struct DmaPtr {
    pub vaddr: *mut (),
    pub paddr: usize,
}

pub struct DmaDef {
    pub vaddr: NonNull<()>,
    /// GEM bus address, which may legitimately be zero.
    pub paddr: usize,
    pub size: usize,
}

use super::{MTU, NUM_BUFS};

use super::rx::DESC_SIZE as RX_DESC_SIZE;
use super::tx::DESC_SIZE as TX_DESC_SIZE;

/// Descriptors use Device memory; packet buffers use cacheable Normal memory.
/// Both regions must be disjoint, remain mapped for the driver's lifetime, and
/// have no aliases with conflicting memory attributes.
pub fn alloc_dma(descriptors: DmaDef, buffers: DmaDef) -> GemDmaPtrs {
    let rx_desc_size = RX_DESC_SIZE * NUM_BUFS;
    let tx_desc_size = TX_DESC_SIZE * NUM_BUFS;
    let buf_size = MTU * NUM_BUFS;
    let desc_size = rx_desc_size + tx_desc_size + TX_DESC_SIZE;

    assert!(descriptors.size >= desc_size);
    assert!(buffers.size >= 2 * buf_size);
    for region in [&descriptors, &buffers] {
        assert_eq!(
            region.vaddr.as_ptr() as usize % super::cache::CACHE_LINE_SIZE,
            0
        );
        assert_eq!(region.paddr % super::cache::CACHE_LINE_SIZE, 0);
        // GEM is configured for 32-bit DMA addresses.
        assert!((region.paddr)
            .checked_add(region.size - 1)
            .is_some_and(|end| end <= u32::MAX as usize));
    }
    debug!(
        "DMA descriptors: {desc_size} bytes; buffers: {} bytes",
        2 * buf_size
    );

    for (desc_base, buf_base) in [
        (
            descriptors.vaddr.as_ptr() as usize,
            buffers.vaddr.as_ptr() as usize,
        ),
        (descriptors.paddr, buffers.paddr),
    ] {
        let desc_end = desc_base.checked_add(descriptors.size).unwrap();
        let buf_end = buf_base.checked_add(buffers.size).unwrap();
        assert!(
            desc_end <= buf_base || buf_end <= desc_base,
            "DMA regions overlap"
        );
    }

    // Virtual addresses remain pointers; physical addresses use integer
    // offsets, including when the descriptor region begins at address zero.
    GemDmaPtrs {
        rx: DmaPtrs {
            desc: DmaPtr {
                vaddr: descriptors.vaddr.as_ptr(),
                paddr: descriptors.paddr,
            },
            buf: DmaPtr {
                vaddr: buffers.vaddr.as_ptr(),
                paddr: buffers.paddr,
            },
        },
        tx: DmaPtrs {
            desc: DmaPtr {
                vaddr: descriptors.vaddr.as_ptr().wrapping_byte_add(rx_desc_size),
                paddr: descriptors.paddr + rx_desc_size,
            },
            buf: DmaPtr {
                vaddr: buffers.vaddr.as_ptr().wrapping_byte_add(buf_size),
                paddr: buffers.paddr + buf_size,
            },
        },
        tx_dummy: DmaPtr {
            vaddr: descriptors
                .vaddr
                .as_ptr()
                .wrapping_byte_add(rx_desc_size + tx_desc_size),
            paddr: descriptors.paddr + rx_desc_size + tx_desc_size,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Layout calculations do not dereference these addresses.
    fn region(vaddr: usize, paddr: usize, size: usize) -> DmaDef {
        DmaDef {
            vaddr: NonNull::new(vaddr as *mut ()).unwrap(),
            paddr,
            size,
        }
    }

    #[test]
    fn separates_descriptors_and_cache_line_isolated_packets() {
        let dma = alloc_dma(
            region(0x80000000, 0x10000000, 4096),
            region(0x80200000, 0x20000000, 0x200000),
        );
        assert_eq!(dma.rx.desc.vaddr as usize, 0x80000000);
        assert_eq!(dma.tx.desc.vaddr as usize, 0x80000400);
        assert_eq!(dma.tx_dummy.vaddr as usize, 0x80000800);
        assert_eq!(dma.rx.buf.vaddr as usize, 0x80200000);
        assert_eq!(dma.tx.buf.vaddr as usize, 0x80232000);
        assert_eq!(dma.tx.desc.paddr as usize, 0x10000400);
        assert_eq!(dma.tx.buf.paddr as usize, 0x20032000);
        for i in 0..NUM_BUFS {
            assert_eq!((dma.rx.buf.vaddr as usize + i * MTU) % 64, 0);
            assert_eq!((dma.tx.buf.vaddr as usize + i * MTU) % 64, 0);
        }
    }

    #[test]
    fn accepts_descriptor_physical_address_zero() {
        let dma = alloc_dma(
            region(0x80000000, 0, 4096),
            region(0x80200000, 0x09400000, 0x200000),
        );
        assert_eq!(dma.rx.desc.paddr, 0);
        assert_eq!(dma.tx.desc.paddr, 0x400);
        assert_eq!(dma.tx_dummy.paddr, 0x800);
        assert_eq!(dma.rx.buf.paddr, 0x09400000);
        assert_eq!(dma.tx.buf.paddr, 0x09432000);
    }

    #[test]
    #[should_panic]
    fn rejects_undersized_descriptors() {
        alloc_dma(
            region(0x80000000, 0x10000000, 2048),
            region(0x80200000, 0x20000000, 0x200000),
        );
    }

    #[test]
    #[should_panic]
    fn rejects_undersized_buffers() {
        alloc_dma(
            region(0x80000000, 0x10000000, 4096),
            region(0x80200000, 0x20000000, MTU * NUM_BUFS),
        );
    }

    #[test]
    #[should_panic]
    fn rejects_unaligned_buffers() {
        alloc_dma(
            region(0x80000000, 0x10000000, 4096),
            region(0x80200001, 0x20000000, 0x200000),
        );
    }

    #[test]
    #[should_panic]
    fn rejects_physical_aliases() {
        alloc_dma(
            region(0x80000000, 0x20000000, 4096),
            region(0x80200000, 0x20000000, 0x200000),
        );
    }

    #[test]
    #[should_panic]
    fn rejects_addresses_outside_gem_32_bit_range() {
        alloc_dma(
            region(0x80000000, 0x10000000, 4096),
            region(0x80200000, 0xfffff000, 0x200000),
        );
    }
}
