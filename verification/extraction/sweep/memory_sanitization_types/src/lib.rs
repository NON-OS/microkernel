// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/hardening/memory_sanitization/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/hardening/memory_sanitization/types.rs"]
pub mod types;

pub fn sanitizationlevel_from_u64(val: u64) -> types::SanitizationLevel {
    types::SanitizationLevel::from_u64(val)
}

pub fn stackcanaryconfig_is_enabled(this: types::StackCanaryConfig) -> bool {
    this.is_enabled()
}

pub fn stackcanaryconfig_get_canary(this: types::StackCanaryConfig) -> u64 {
    this.get_canary()
}

pub fn stackcanaryconfig_get_frequency(this: types::StackCanaryConfig) -> u32 {
    this.get_frequency()
}

pub fn stackcanaryconfig_verify(this: types::StackCanaryConfig, value: u64) -> bool {
    this.verify(value)
}

pub fn sanitizationstats_get_bytes_sanitized(this: types::SanitizationStats) -> usize {
    this.get_bytes_sanitized()
}

pub fn sanitizationstats_get_call_count(this: types::SanitizationStats) -> usize {
    this.get_call_count()
}

pub fn sanitizationstats_is_canary_enabled(this: types::SanitizationStats) -> bool {
    this.is_canary_enabled()
}

pub fn sanitizationstats_avg_bytes_per_call(this: types::SanitizationStats) -> usize {
    this.avg_bytes_per_call()
}

