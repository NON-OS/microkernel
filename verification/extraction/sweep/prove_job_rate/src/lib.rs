// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/userspace/init/prove_job/rate.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/userspace/init/prove_job/rate.rs"]
pub mod rate;

pub fn rate_new() -> rate::Rate {
    rate::Rate::new()
}

