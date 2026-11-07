// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/nvme_capsule/client/lba_map.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/nvme_capsule/client/lba_map.rs"]
pub mod lba_map;

pub fn addressable(lba_size: u32, buffer: u32) -> bool {
    lba_map::addressable(lba_size, buffer)
}

pub fn max_transfer_bytes(mdts: u8, buffer: u64) -> u64 {
    lba_map::max_transfer_bytes(mdts, buffer)
}

pub fn lbas_per_request(mdts: u8, lba_size: u32, buffer: u64) -> u64 {
    lba_map::lbas_per_request(mdts, lba_size, buffer)
}

