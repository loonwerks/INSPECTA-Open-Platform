//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

pub mod sizes {
    pub const DMA_DESCRIPTORS: usize = 0x1000;
    pub const DMA_BUFFERS: usize = 0x20_0000;
}

#[cfg(feature = "sel4")]
pub mod log {
    use sel4::debug_print;
    use sel4_logging::{LevelFilter, Logger, LoggerBuilder};

    const LOG_LEVEL: LevelFilter = {
        // LevelFilter::Trace
        // LevelFilter::Debug
        LevelFilter::Info
        // LevelFilter::Warn
    };

    pub static LOGGER: Logger = LoggerBuilder::const_default()
        .level_filter(LOG_LEVEL)
        .write(|s| debug_print!("{}", s))
        .build();
}
