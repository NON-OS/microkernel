// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/ipc/recv_meta.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/ipc/recv_meta.rs"]
pub mod recv_meta;

pub fn meta_len_ok(meta_len: u64) -> bool {
    recv_meta::meta_len_ok(meta_len)
}

