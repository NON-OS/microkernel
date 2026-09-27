// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/irq/reserved.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/irq/reserved.rs"]
pub mod reserved;

pub fn reserve(gsi: u32) {
    reserved::reserve(gsi)
}

pub fn is_reserved(gsi: u32) -> bool {
    reserved::is_reserved(gsi)
}

