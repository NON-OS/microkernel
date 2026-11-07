// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/kernel_core/process_spawn/capsule_spawn/runner/secret_log.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/kernel_core/process_spawn/capsule_spawn/runner/secret_log.rs"]
pub mod secret_log;

pub fn unlogged(caps: u64, dev: bool) -> u64 {
    secret_log::unlogged(caps, dev)
}

