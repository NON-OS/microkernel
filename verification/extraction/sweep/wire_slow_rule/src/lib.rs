// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/block_device/wire/slow_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/block_device/wire/slow_rule.rs"]
pub mod slow_rule;

pub fn said(status: i32, answered_ms: u64) -> bool {
    slow_rule::said(status, answered_ms)
}

pub fn lost_said(timed_out: bool, answered_ms: u64) -> bool {
    slow_rule::lost_said(timed_out, answered_ms)
}

