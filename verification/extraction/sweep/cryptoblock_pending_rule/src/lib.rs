// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/fs/cryptoblock/pending_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/fs/cryptoblock/pending_rule.rs"]
pub mod pending_rule;

pub fn must_send(start: u64, held: u64, at: u64, cap: u64) -> bool {
    pending_rule::must_send(start, held, at, cap)
}

