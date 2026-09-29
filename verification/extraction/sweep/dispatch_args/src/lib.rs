// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/dispatch/args.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/dispatch/args.rs"]
pub mod args;

pub fn args_new(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> args::Args {
    args::Args::new(a0, a1, a2, a3, a4, a5)
}

