// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/pio/fixed_line.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/pio/fixed_line.rs"]
pub mod fixed_line;

pub fn owns_line(primary_fixed: bool, secondary_fixed: bool, irq: u32) -> bool {
    fixed_line::owns_line(primary_fixed, secondary_fixed, irq)
}

