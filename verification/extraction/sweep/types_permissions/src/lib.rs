// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/mmu/types/permissions.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/mmu/types/permissions.rs"]
pub mod permissions;

pub fn pagepermissions_kernel_ro() -> permissions::PagePermissions {
    permissions::PagePermissions::kernel_ro()
}

pub fn pagepermissions_kernel_rw() -> permissions::PagePermissions {
    permissions::PagePermissions::kernel_rw()
}

pub fn pagepermissions_kernel_rx() -> permissions::PagePermissions {
    permissions::PagePermissions::kernel_rx()
}

pub fn pagepermissions_device() -> permissions::PagePermissions {
    permissions::PagePermissions::device()
}

