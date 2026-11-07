// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/holder/rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/holder/rule.rs"]
pub mod rule;

pub fn group_of(tgid: u32, pid: u32) -> u32 {
    rule::group_of(tgid, pid)
}

