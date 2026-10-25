// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::limits::{MAX_CPUS, MAX_MEMORY_REGIONS};
use super::memory::MemoryRegion;
use crate::arch::aarch64::gic::GicVersion;
use crate::arch::fdt::find::msi::MsiController;
use crate::arch::fdt::find::smmu::{IommuMap, SmmuInfo};
use crate::arch::fdt::find::uart::UartInfo;
use crate::arch::fdt::reserved_set::Reserved;

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub ram_base: u64,
    pub ram_size: u64,
    pub kernel_base: u64,
    pub kernel_size: u64,
    pub dtb_base: u64,
    pub dtb_size: u64,
    pub uart: UartInfo,
    /// The GIC the tree names; `None` without a tree, and the CPU decides.
    pub gic_version: Option<GicVersion>,
    pub gic_dist_base: u64,
    /// GICv3's redistributor region.
    pub gic_redist_base: u64,
    /// GICv2's CPU interface.
    pub gic_cpu_base: u64,
    pub cpu_count: u32,
    /// Each CPU's MPIDR affinity in tree order: how PSCI and the GIC name it.
    pub cpu_affinities: [u64; MAX_CPUS],
    pub cpu_affinity_count: usize,
    /// Per CPU, same index: the spin-table address it waits on; `None`, PSCI.
    pub cpu_release: [Option<u64>; MAX_CPUS],
    pub timer_phys_intid: u32,
    pub timer_virt_intid: u32,
    /// The PCI host bridge's config and I/O windows; zero with no PCI at all.
    pub pci_ecam_base: u64,
    pub pci_ecam_size: u64,
    pub pci_io_cpu_base: u64,
    pub pci_io_port_base: u64,
    pub pci_io_size: u64,
    /// The buses the ECAM window covers; its first megabyte is the first bus.
    pub pci_bus_start: u8,
    pub pci_bus_end: u8,
    /// The memory window the host bridge forwards to its devices. No firmware
    /// assigned a BAR before a tree boot, so the kernel assigns them from it.
    pub pci_mmio_base: u64,
    pub pci_mmio_size: u64,
    /// MMIO base of the PL031 real-time clock, zero when the board has none.
    pub rtc_base: u64,
    /// The SMMUv3 and the host bridge's `iommu-map`, when the tree has one.
    pub smmu: Option<(SmmuInfo, Option<IommuMap>)>,
    /// The MSI controller the host bridge's functions write to, and its map.
    pub msi: Option<(MsiController, IommuMap)>,
    /// Fixed storage: filled in `kernel_entry`, before the heap, so no `Vec`.
    pub memory_regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    pub memory_region_count: usize,
    /// Inside RAM but not the allocator's: see `Reserved`.
    pub reserved: Reserved,
    /// The initrd the loader placed, `[start, end)`, if it placed one.
    pub initrd: Option<(u64, u64)>,
}
