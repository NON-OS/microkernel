// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/types/permissions/flags.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub mod memory;

pub fn pagepermissions_empty() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::empty()
}

pub fn pagepermissions_from_bits(bits: u32) -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::from_bits(bits)
}

pub fn pagepermissions_bits(this: crate::memory::paging::types::permissions::flags::PagePermissions) -> u32 {
    this.bits()
}

pub fn pagepermissions_contains(this: crate::memory::paging::types::permissions::flags::PagePermissions, other: crate::memory::paging::types::permissions::flags::PagePermissions) -> bool {
    this.contains(other)
}

pub fn pagepermissions_union(this: crate::memory::paging::types::permissions::flags::PagePermissions, other: crate::memory::paging::types::permissions::flags::PagePermissions) -> crate::memory::paging::types::permissions::flags::PagePermissions {
    this.union(other)
}

pub fn pagepermissions_remove(this: crate::memory::paging::types::permissions::flags::PagePermissions, other: crate::memory::paging::types::permissions::flags::PagePermissions) -> crate::memory::paging::types::permissions::flags::PagePermissions {
    this.remove(other)
}

pub fn pagepermissions_insert(this: crate::memory::paging::types::permissions::flags::PagePermissions, other: crate::memory::paging::types::permissions::flags::PagePermissions) -> crate::memory::paging::types::permissions::flags::PagePermissions {
    this.insert(other)
}

pub fn pagepermissions_is_wx_violation(this: crate::memory::paging::types::permissions::flags::PagePermissions) -> bool {
    this.is_wx_violation()
}

