// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/claim/wipe/bits_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/claim/wipe/bits_rule.rs"]
pub mod bits_rule;

pub fn not_ready(id: u32) -> bool {
    bits_rule::not_ready(id)
}

pub fn devctl_after(before: u16) -> u16 {
    bits_rule::devctl_after(before)
}

pub fn devctl2_after(before: u16, boot: u16) -> u16 {
    bits_rule::devctl2_after(before, boot)
}

pub fn pmcsr_to(read: u16, state: u16) -> u16 {
    bits_rule::pmcsr_to(read, state)
}

