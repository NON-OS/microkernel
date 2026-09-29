// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/types/permissions/convert.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub mod memory;

pub fn pagepermissions_to_pte_flags(this: crate::memory::paging::types::permissions::flags::PagePermissions) -> u64 {
    this.to_pte_flags()
}

pub fn pagepermissions_kernel_ro() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::kernel_ro()
}

pub fn pagepermissions_kernel_rw() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::kernel_rw()
}

pub fn pagepermissions_kernel_rx() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::kernel_rx()
}

pub fn pagepermissions_user_ro() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::user_ro()
}

pub fn pagepermissions_user_rw() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::user_rw()
}

pub fn pagepermissions_user_rx() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::user_rx()
}

pub fn pagepermissions_device() -> crate::memory::paging::types::permissions::flags::PagePermissions {
    crate::memory::paging::types::permissions::flags::PagePermissions::device()
}

