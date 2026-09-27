// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/capabilities/chain/error.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/capabilities/chain/error.rs"]
pub mod error;

pub fn chainerror_is_recoverable(this: error::ChainError) -> bool {
    this.is_recoverable()
}

