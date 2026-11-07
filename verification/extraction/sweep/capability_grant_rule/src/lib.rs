// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/capability/grant_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/capability/grant_rule.rs"]
pub mod grant_rule;

pub fn may_grant(held: u64, admin: u64, mask: u64) -> bool {
    grant_rule::may_grant(held, admin, mask)
}

pub fn within_envelope(mask: u64, envelope: u64) -> bool {
    grant_rule::within_envelope(mask, envelope)
}

pub fn may_revoke(held: u64, admin: u64) -> bool {
    grant_rule::may_revoke(held, admin)
}

pub fn revoke_decision(
    same_process: bool,
    caller_admin: bool,
    caller_is_init: bool,
    caller_is_parent: bool,
    target_protected: bool,
) -> bool {
    grant_rule::revoke_decision(
        same_process,
        caller_admin,
        caller_is_init,
        caller_is_parent,
        target_protected,
    )
}
