// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/services/caps/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod capabilities;

pub mod services;

pub fn servicecap_new(bits: u64, owner: u32) -> crate::services::caps::types::ServiceCap {
    crate::services::caps::types::ServiceCap::new(bits, owner)
}

pub fn servicecap_with_expiry(bits: u64, owner: u32, expires_ms: u64) -> crate::services::caps::types::ServiceCap {
    crate::services::caps::types::ServiceCap::with_expiry(bits, owner, expires_ms)
}

pub fn servicecap_has(this: crate::services::caps::types::ServiceCap, cap: u64) -> bool {
    this.has(cap)
}

pub fn servicecap_is_expired(this: crate::services::caps::types::ServiceCap, now_ms: u64) -> bool {
    this.is_expired(now_ms)
}

