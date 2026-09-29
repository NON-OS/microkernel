// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/security/lba.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn is_lba_in_partition(lba: u64, partition_start: u64, partition_size: u64) -> bool {
    crate::drivers::security::lba::is_lba_in_partition(lba, partition_start, partition_size)
}

