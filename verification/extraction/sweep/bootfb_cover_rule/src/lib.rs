// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/bootfb/cover_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/bootfb/cover_rule.rs"]
pub mod cover_rule;

pub fn covers(mine: bool, any: bool, display: bool, single_display: bool) -> bool {
    cover_rule::covers(mine, any, display, single_display)
}

