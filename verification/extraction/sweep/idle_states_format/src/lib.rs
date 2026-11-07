// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/idle_states/format.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/idle_states/format.rs"]
pub mod format;

pub fn is_standby(param: u32, extended: bool) -> bool {
    format::is_standby(param, extended)
}

