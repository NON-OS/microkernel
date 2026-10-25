// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the std layer's thread registers. `thread_regs` is the
//! layer's own file; `enter` and `leave` stand in for the kernel's side.

#![allow(dead_code)]

#[path = "../../../toolchain/nonos-std/sys/pal/nonos/thread_regs.rs"]
pub mod thread_regs;

#[cfg(test)]
mod enter;
#[cfg(test)]
mod tls_tests;
#[cfg(test)]
mod trampoline_tests;
