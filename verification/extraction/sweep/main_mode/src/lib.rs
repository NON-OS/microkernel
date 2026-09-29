// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/boot/main/mode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/boot/main/mode.rs"]
pub mod mode;

pub fn is_microkernel() -> bool {
    mode::is_microkernel()
}

