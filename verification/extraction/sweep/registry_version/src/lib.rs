// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/elf/embedded/registry/version.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/elf/embedded/registry/version.rs"]
pub mod version;

pub fn libraryversion_new(major: u32, minor: u32, patch: u32) -> version::LibraryVersion {
    version::LibraryVersion::new(major, minor, patch)
}

