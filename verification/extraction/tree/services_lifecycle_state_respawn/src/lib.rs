// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/services/lifecycle/state/respawn.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod services;

pub fn capsulestate_restart_count(this: crate::services::lifecycle::state::types::CapsuleState) -> u32 {
    this.restart_count()
}

pub fn capsulestate_last_exit_ms(this: crate::services::lifecycle::state::types::CapsuleState) -> u64 {
    this.last_exit_ms()
}

pub fn capsulestate_record_exit(this: crate::services::lifecycle::state::types::CapsuleState, now_ms: u64) {
    this.record_exit(now_ms)
}

pub fn capsulestate_should_respawn(this: crate::services::lifecycle::state::types::CapsuleState, now_ms: u64) -> bool {
    this.should_respawn(now_ms)
}

pub fn capsulestate_set_max_restarts(this: crate::services::lifecycle::state::types::CapsuleState, value: u32) {
    this.set_max_restarts(value)
}

pub fn capsulestate_set_debounce_ms(this: crate::services::lifecycle::state::types::CapsuleState, value: u64) {
    this.set_debounce_ms(value)
}

