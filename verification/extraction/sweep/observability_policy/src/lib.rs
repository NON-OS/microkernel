// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/observability/policy.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/observability/policy.rs"]
pub mod policy;

pub fn is_production_mode() -> bool {
    policy::is_production_mode()
}

pub fn set_production_mode(enabled: bool) {
    policy::set_production_mode(enabled)
}

pub fn should_log_debug() -> bool {
    policy::should_log_debug()
}

pub fn should_emit_serial() -> bool {
    policy::should_emit_serial()
}

