// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/shown.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/shown.rs"]
pub mod shown;

pub fn shown_base(base: u64, kind: u8, held: bool) -> u64 {
    shown::shown_base(base, kind, held)
}

