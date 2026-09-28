// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/fd_types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/fd_types.rs"]
pub mod fd_types;

pub fn fdentry_with_pipe(pipe_id: usize, is_read: bool) -> fd_types::FdEntry {
    fd_types::FdEntry::with_pipe(pipe_id, is_read)
}

pub fn fdentry_is_cloexec(this: fd_types::FdEntry) -> bool {
    this.is_cloexec()
}

