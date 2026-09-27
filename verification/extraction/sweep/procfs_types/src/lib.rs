// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/fs/procfs/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/fs/procfs/types.rs"]
pub mod types;

pub fn procinode_root() -> types::ProcInode {
    types::ProcInode::root()
}

pub fn procinode_for_pid(ino: u64, pid: i32) -> types::ProcInode {
    types::ProcInode::for_pid(ino, pid)
}

