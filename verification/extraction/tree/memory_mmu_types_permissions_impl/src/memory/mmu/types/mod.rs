// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/memory/mmu/types/permissions.rs"]
pub mod permissions;

#[path = "../../../../../../../../src/memory/mmu/types/permissions_impl.rs"]
pub mod permissions_impl;

#[path = "../../../../../../../../src/memory/mmu/types/pte.rs"]
pub mod pte;

pub use permissions::PagePermissions;
pub use pte::PageTableEntry;
