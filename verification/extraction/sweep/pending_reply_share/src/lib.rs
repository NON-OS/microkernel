// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/ipc/pending_reply/share.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/ipc/pending_reply/share.rs"]
pub mod share;

pub fn admits(queued: usize, mine: usize) -> bool {
    share::admits(queued, mine)
}

