// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/ipc/call/silence.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/ipc/call/silence.rs"]
pub mod silence;

pub fn silence_new(window_ms: u64) -> silence::Silence {
    silence::Silence::new(window_ms)
}

