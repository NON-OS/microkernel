// NONOS Operating System (AGPL-3.0-or-later)
//! `crate::arch::aarch64::mmu` rebuilt from the kernel's files, so they are
//! included unmodified. Only the TLB flush and the image's linker symbols are
//! stand-ins: neither exists on the host.

// The kernel's files carry what the rest of the kernel uses of them too.
#![allow(dead_code)]

// The three below restate their kernel module files' declarations: a
// `#[path]` file not named mod.rs does not lend its children its directory.
#[path = "../../../../../../src/arch/aarch64/mmu/attributes"]
pub mod attributes {
    mod constructors;
    mod descriptor;
    mod kind;
    mod page;
    mod pte;

    pub use kind::MemoryType;
    pub use page::PageAttributes;
    pub use pte::*;
}
#[path = "../../../../../../src/arch/aarch64/mmu/boot_devices.rs"]
mod boot_devices;
#[path = "../../../../../../src/arch/aarch64/mmu/direct_map.rs"]
mod direct_map;
#[path = "../../../../../../src/arch/aarch64/mmu/geometry.rs"]
mod geometry;
#[path = "../../../../../../src/arch/aarch64/mmu/granule"]
pub mod granule {
    mod align;
    mod constants;
    mod index;
    mod kind;
    mod size;

    pub use kind::Granule;
}
mod image_bounds;
#[path = "../../../../../../src/arch/aarch64/mmu/image_map.rs"]
mod image_map;
#[path = "../../../../../../src/arch/aarch64/mmu/l2_pool.rs"]
mod l2_pool;
#[path = "../../../../../../src/arch/aarch64/mmu/l3_tables.rs"]
mod l3_tables;
pub mod leaves;
#[path = "../../../../../../src/arch/aarch64/mmu/map.rs"]
mod map;
pub mod probe;
#[path = "../../../../../../src/arch/aarch64/mmu/ram_map.rs"]
mod ram_map;
#[path = "../../../../../../src/arch/aarch64/mmu/state.rs"]
mod state;
#[path = "../../../../../../src/arch/aarch64/mmu/table"]
pub mod table {
    mod entry;
    mod page_table;
    mod walk;

    pub use page_table::PageTable;
    pub use walk::walk_page_tables;
}
#[path = "../../../../../../src/arch/aarch64/mmu/tables_build.rs"]
mod tables_build;
pub mod text_guard;
#[path = "../../../../../../src/arch/aarch64/mmu/unmap.rs"]
mod unmap;

pub use attributes::{MemoryType, PageAttributes};

mod tlb {
    /// The host has no TLB; the tables are read back directly.
    pub fn flush_tlb_page(_virt: u64) {}
}
