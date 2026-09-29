// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/fs/vfs/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod fs;

pub fn openflags_empty() -> crate::fs::vfs::types::OpenFlags {
    crate::fs::vfs::types::OpenFlags::empty()
}

pub fn openflags_from_bits(bits: u32) -> crate::fs::vfs::types::OpenFlags {
    crate::fs::vfs::types::OpenFlags::from_bits(bits)
}

pub fn openflags_bits(this: crate::fs::vfs::types::OpenFlags) -> u32 {
    this.bits()
}

pub fn openflags_contains(this: crate::fs::vfs::types::OpenFlags, other: crate::fs::vfs::types::OpenFlags) -> bool {
    this.contains(other)
}

pub fn openflags_is_readable(this: crate::fs::vfs::types::OpenFlags) -> bool {
    this.is_readable()
}

pub fn openflags_is_writable(this: crate::fs::vfs::types::OpenFlags) -> bool {
    this.is_writable()
}

