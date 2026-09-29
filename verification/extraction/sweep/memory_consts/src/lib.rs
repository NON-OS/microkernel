// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/memory/consts.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/memory/consts.rs"]
pub mod consts;

pub fn is_user_space(addr: u64, len: usize) -> bool {
    consts::is_user_space(addr, len)
}

