// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/irq/bound.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/irq/bound.rs"]
pub mod bound;

pub fn wait_ms(timeout_ms: u64) -> u64 {
    bound::wait_ms(timeout_ms)
}

