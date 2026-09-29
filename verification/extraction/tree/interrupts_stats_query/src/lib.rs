// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/interrupts/stats/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod interrupts;

pub fn reset_stats() {
    crate::interrupts::stats::query::reset_stats()
}

