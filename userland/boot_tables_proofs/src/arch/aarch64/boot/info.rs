// NONOS Operating System (AGPL-3.0-or-later)
//! The fields of `BootInfo` the table builder reads, and the kernel's own
//! memory region types.

#[path = "../../../../../../src/arch/aarch64/boot/info/memory.rs"]
mod memory;

pub use memory::{MemoryRegion, MemoryType};

/// A window by base and size: the console's and the MSI controller's shape.
#[derive(Debug, Clone, Copy, Default)]
pub struct Window {
    pub base: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Default)]
pub struct BootInfo {
    pub ram_base: u64,
    pub ram_size: u64,
    pub regions: Vec<MemoryRegion>,
    pub uart: Window,
    pub gic_dist_base: u64,
    pub gic_cpu_base: u64,
    pub gic_redist_base: u64,
    pub msi: Option<(Window, ())>,
    pub pci_io_cpu_base: u64,
    pub pci_io_size: u64,
    pub pci_mmio_base: u64,
    pub pci_mmio_size: u64,
    pub rtc_base: u64,
}

impl BootInfo {
    pub fn memory_map(&self) -> &[MemoryRegion] {
        &self.regions
    }
}
