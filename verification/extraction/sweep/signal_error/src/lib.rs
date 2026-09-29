// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/signal/error.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/signal/error.rs"]
pub mod error;

pub fn signalerror_as_errno(this: error::SignalError) -> i32 {
    this.as_errno()
}

