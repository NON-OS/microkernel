// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/fs/blockfs/header_lba.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod fs;

pub fn header_lba(generation: u64) -> u64 {
    crate::fs::blockfs::header_lba::header_lba(generation)
}

