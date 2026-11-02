//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

use super::{cache, DataBuf, MTU, NUM_BUFS};
use core::ops::{Deref, DerefMut};

mod descriptor;
use descriptor::Descriptor;

use super::DmaPtrs;

pub const DESC_SIZE: usize = core::mem::size_of::<Descriptor>();

pub struct RxRing {
    curr_entry: usize,
    buffer: DataBuf,
    entries: *mut [Descriptor; NUM_BUFS],
}

impl RxRing {
    pub fn new(dma_ptrs: &DmaPtrs) -> Self {
        let entries = dma_ptrs.desc.vaddr.cast();
        let buf_ptr = dma_ptrs.buf.vaddr.cast();
        let mut ring = Self {
            curr_entry: 0,
            buffer: DataBuf::new(buf_ptr),
            entries,
        };
        cache::check_line_size();
        // Clear any dirty/stale cache lines before the first DMA write.
        cache::clean_invalidate(dma_ptrs.buf.vaddr.cast(), MTU * NUM_BUFS);
        ring.setup(dma_ptrs.buf.paddr);
        cache::barrier();
        ring
    }

    fn entries(&self) -> *mut [Descriptor; NUM_BUFS] {
        self.entries
    }

    fn setup(&mut self, buffers_paddr: usize) {
        for (i, entry) in self.iter_mut().enumerate() {
            entry.set_addr(buffers_paddr + (i * MTU));
            entry.mark_done();
        }
        self.last_mut().unwrap().mark_last();
    }

    pub fn next_entry_available(&self) -> bool {
        self.get(self.curr_entry).unwrap().is_available()
    }

    pub fn recv_next(&mut self) -> &mut [u8] {
        // The caller has observed GEM returning descriptor ownership. Finish
        // that read before invalidating speculative/stale cached packet data.
        // clean_invalidate already provides the required leading barrier.
        let packet = self.buffer.get(self.curr_entry);
        cache::clean_invalidate(packet.as_mut_ptr(), packet.len());
        packet
    }

    pub fn mark_done(&mut self) {
        let curr_entry = self.curr_entry;
        let packet = self.buffer.get(curr_entry);
        cache::clean_invalidate(packet.as_mut_ptr(), packet.len());
        self.get_mut(curr_entry).unwrap().mark_done();
        cache::barrier();
        let entries_len = self.len();
        self.curr_entry = (self.curr_entry + 1) % entries_len;
    }
}

impl Deref for RxRing {
    type Target = [Descriptor; NUM_BUFS];

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.entries() }
    }
}

impl DerefMut for RxRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.entries() }
    }
}
