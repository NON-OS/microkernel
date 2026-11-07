// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/services/lifecycle/transport_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/services/lifecycle/transport_rule.rs"]
pub mod transport_rule;

pub fn takes_over(pid: u32, ended: bool, on_cpu: bool) -> bool {
    transport_rule::takes_over(pid, ended, on_cpu)
}

