// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/fs/procfs/pid_inode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/fs/procfs/pid_inode.rs"]
pub mod pid_inode;


pub fn pid_dir_inode(pid: i32) -> Option<u64> {
    pid_inode::pid_dir_inode(pid)
}
