// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/mmio/forbidden.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/mmio/forbidden.rs"]
pub mod forbidden;

pub fn is_ram(memory_type: u32) -> bool {
    forbidden::is_ram(memory_type)
}

