// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/pio/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/pio/types.rs"]
pub mod types;

pub fn piowidth_bytes(this: types::PioWidth) -> u16 {
    this.bytes()
}

