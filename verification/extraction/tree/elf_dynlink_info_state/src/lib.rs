// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/elf/dynlink/info/state.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod elf;

pub mod memory;

pub fn dynlinkinfo_new() -> crate::elf::dynlink::info::state::DynLinkInfo {
    crate::elf::dynlink::info::state::DynLinkInfo::new()
}

