//! ZCU102 Cortex-A53 DMA cache maintenance to the point of coherency.
//!
//! Buffers are private, whole cache lines. The CPU must not touch a buffer
//! while GEM owns it. In particular, the post-RX CIVAC must never clean dirty
//! CPU data over a packet written by GEM. Preparing RX before ownership is
//! transferred, and forbidding CPU access until completion, ensures this.

pub const CACHE_LINE_SIZE: usize = 64;

pub fn check_line_size() {
    #[cfg(target_arch = "aarch64")]
    {
        let ctr: usize;
        unsafe {
            core::arch::asm!("mrs {}, ctr_el0", out(reg) ctr, options(nomem, nostack));
        }
        assert_eq!(4usize << ((ctr >> 16) & 0xf), CACHE_LINE_SIZE);
    }
    #[cfg(not(target_arch = "aarch64"))]
    panic!("GEM cache maintenance requires AArch64");
}

/// Complete CPU memory accesses and maintenance before crossing ownership.
#[inline]
pub fn barrier() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
    #[cfg(not(target_arch = "aarch64"))]
    panic!("GEM cache maintenance requires AArch64");
}

/// Clean only the lines containing the TX packet before GEM reads it.
/// The caller owns the whole final line, including bytes after `len`.
pub fn clean(ptr: *mut u8, len: usize) {
    maintain(ptr, len, false);
}

/// Remove cached RX data before GEM writes and after it completes.
/// CIVAC is usable at EL0 with seL4's UCI configuration; IVAC is privileged.
pub fn clean_invalidate(ptr: *mut u8, len: usize) {
    maintain(ptr, len, true);
}

fn maintain(ptr: *mut u8, len: usize, invalidate: bool) {
    // Every caller passes a cache-line-aligned, isolated buffer. Round the
    // packet length up so that its final partial line is maintained too.
    let start = ptr as usize;
    assert_eq!(start % CACHE_LINE_SIZE, 0);
    let end = start.checked_add(rounded_len(len)).unwrap();
    barrier();
    let mut addr = start;
    while addr < end {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            // No `nomem`: these instructions must also order compiler accesses.
            if invalidate {
                core::arch::asm!("dc civac, {}", in(reg) addr, options(nostack, preserves_flags));
            } else {
                core::arch::asm!("dc cvac, {}", in(reg) addr, options(nostack, preserves_flags));
            }
        }
        #[cfg(not(target_arch = "aarch64"))]
        let _ = invalidate;
        addr += CACHE_LINE_SIZE;
    }
    barrier();
}

fn rounded_len(len: usize) -> usize {
    len.checked_add(CACHE_LINE_SIZE - 1).unwrap() & !(CACHE_LINE_SIZE - 1)
}

#[cfg(test)]
mod tests {
    use super::rounded_len;

    #[test]
    fn covers_packet_tail_without_cleaning_the_entire_buffer() {
        for (len, expected) in [
            (0, 0),
            (1, 64),
            (64, 64),
            (65, 128),
            (342, 384),
            (1600, 1600),
        ] {
            assert_eq!(rounded_len(len), expected);
        }
    }
}
