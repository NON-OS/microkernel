// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/foreign/peer_lock/stripe.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/foreign/peer_lock/stripe.rs"]
pub mod stripe;

pub fn stripe_of(asid: u32) -> usize {
    stripe::stripe_of(asid)
}

