// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/pio/access/reset_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/pio/access/reset_rule.rs"]
pub mod reset_rule;

pub fn resets_machine(port: u16, width: u8, value: u32) -> bool {
    reset_rule::resets_machine(port, width, value)
}

