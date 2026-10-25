// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the thread registers of the std layer and of libc.
//! `thread_regs` and `libc_entry` are their own files; `enter` and `leave`
//! stand in for the kernel's side, and `unistd` for libc's exit.

#![allow(dead_code)]

#[path = "../../../toolchain/nonos-std/sys/pal/nonos/thread_regs.rs"]
pub mod thread_regs;

#[cfg(test)]
mod enter;
#[cfg(test)]
#[path = "../../libc/src/thread/entry.rs"]
mod libc_entry;
#[cfg(test)]
mod libc_entry_tests;
#[cfg(test)]
mod unistd {
    /// libc's thread exit, here the bare Linux one.
    pub fn mk_exit(_status: i32) -> ! {
        crate::enter::leave()
    }
}
#[cfg(test)]
mod tls_tests;
#[cfg(test)]
mod trampoline_tests;
