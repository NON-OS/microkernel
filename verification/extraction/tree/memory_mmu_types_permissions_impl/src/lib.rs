// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/mmu/types/permissions_impl.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn pagepermissions_is_wx_violation(this: crate::memory::mmu::types::permissions::PagePermissions) -> bool {
    this.is_wx_violation()
}

