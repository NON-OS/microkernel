// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/contract/args.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/contract/args.rs"]
pub mod args;

pub fn syscallargs_arg(this: args::SyscallArgs, i: usize) -> u64 {
    this.arg(i)
}

