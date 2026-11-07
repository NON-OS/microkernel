// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/kernel_core/surface_registry/share/rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/kernel_core/surface_registry/share/rule.rs"]
pub mod rule;

pub fn shares_new(owner_group: u32) -> rule::Shares {
    rule::Shares::new(owner_group)
}

pub fn shares_may_attach(this: rule::Shares, receiver: u32) -> bool {
    this.may_attach(receiver)
}

