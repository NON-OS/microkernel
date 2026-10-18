// NONOS Operating System (AGPL-3.0-or-later)
//! What a test may do to the tables: build them once, walk them, and call the
//! runtime map and unmap.

use super::attributes::{PTE_ADDR_MASK, PTE_ATTR_INDX_MASK, PTE_PXN, PTE_TABLE};
use super::granule::Granule;
use super::table::walk_page_tables;
use super::PageAttributes;
use super::{direct_map, geometry, map, state, tables_build, unmap};
use crate::arch::aarch64::boot::info::BootInfo;

pub use super::alias::{high_mapped, is_text, text_pages};
pub use super::image_bounds::{IMAGE, TEXT};
pub use super::text_guard::GUARDED;
pub const KERNEL_SPACE_START: u64 = geometry::KERNEL_SPACE_START;

/// A translation: output address of the whole entry, its level, and the raw
/// descriptor, read with the kernel's walker and then by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Leaf {
    pub output: u64,
    pub level: usize,
    pub descriptor: u64,
}

impl Leaf {
    pub fn attr_index(&self) -> u64 {
        (self.descriptor & PTE_ATTR_INDX_MASK) >> 2
    }
    /// Whether EL1 may run code from it.
    pub fn executable(&self) -> bool {
        self.descriptor & PTE_PXN == 0
    }
}

pub fn build(info: &BootInfo) {
    // SAFETY: each test binary calls this once, on one thread.
    unsafe { tables_build::build(info) }
}

pub fn walk(virt: u64) -> Option<Leaf> {
    // SAFETY: the tables are only read here, between calls that write them.
    let root = unsafe { &*state::l0() };
    // SAFETY: a table address the builder wrote, a host pointer here.
    let host = |pa: u64| Some(unsafe { &*(pa as *const _) });
    let (output, level) = walk_page_tables(root, virt, Granule::G4K, host)?;
    let mut table = root;
    for depth in 0..level {
        let entry = table.entry(Granule::G4K.index_at_level(virt, depth));
        assert!(entry & PTE_TABLE != 0, "a walk past level {depth} went through a table");
        // SAFETY: a table address the builder wrote, a host pointer here.
        table = unsafe { &*((entry & PTE_ADDR_MASK) as *const _) };
    }
    let descriptor = table.entry(Granule::G4K.index_at_level(virt, level));
    Some(Leaf { output, level, descriptor })
}

/// The kernel's resume check (`mmu::one_to_one`) over the built root.
pub fn executable_one_to_one(start: u64, end: u64) -> Result<(), &'static [u8]> {
    // SAFETY: the tables are only read here, between calls that write them.
    let root = unsafe { &*state::l0() };
    // SAFETY: a table address the builder wrote, a host pointer here.
    let host = |pa: u64| Some(unsafe { &*(pa as *const _) });
    super::one_to_one::executable_one_to_one(root, start, end, host)
}

/// The AttrIndx the kernel gives registers and ordinary RAM.
pub fn device_attr() -> u64 {
    (PageAttributes::device().to_descriptor_bits() & PTE_ATTR_INDX_MASK) >> 2
}

pub fn ram_attr() -> u64 {
    (PageAttributes::kernel_data().to_descriptor_bits() & PTE_ATTR_INDX_MASK) >> 2
}

pub fn direct_map_end() -> u64 {
    direct_map::direct_map_end()
}

pub fn map_page(virt: u64, phys: u64) {
    map::map_page(virt, phys, PageAttributes::kernel_data());
}

pub fn unmap_page(virt: u64) {
    unmap::unmap_page(virt);
}
