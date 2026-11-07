// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/security/spectre/cores.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/security/spectre/cores.rs"]
pub mod cores;

pub fn is_arm(implementer: u8) -> bool {
    cores::is_arm(implementer)
}

pub fn in_order(implementer: u8, part: u16) -> bool {
    cores::in_order(implementer, part)
}

pub fn meltdown(implementer: u8, part: u16) -> bool {
    cores::meltdown(implementer, part)
}

