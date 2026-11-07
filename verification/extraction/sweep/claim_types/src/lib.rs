// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/claim/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/claim/types.rs"]
pub mod types;

pub fn claim_making(pid: u32, device_id: u64, epoch: u64) -> types::Claim {
    types::Claim::making(pid, device_id, epoch)
}

