// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/zerostate/volume_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/zerostate/volume_rule.rs"]
pub mod volume_rule;

pub fn drop_at_arm(held_for_disk: bool) -> bool {
    volume_rule::drop_at_arm(held_for_disk)
}

pub fn may_write(ram: bool, keeping: bool) -> bool {
    volume_rule::may_write(ram, keeping)
}

