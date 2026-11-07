// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/confine/decide.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/confine/decide.rs"]
pub mod decide;

pub fn opted_in(opt_in: bool, production: bool) -> bool {
    decide::opted_in(opt_in, production)
}

