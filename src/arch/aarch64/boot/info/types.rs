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

/// As many regions as the device tree walker reads in one pass.
pub(super) const MAX_MEMORY_REGIONS: usize = 8;

/// As many CPUs as the device tree walker reads in one pass.
pub(super) const MAX_CPUS: usize = 64;

use super::memory::MemoryRegion;

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub ram_base: u64,
    pub ram_size: u64,
    pub kernel_base: u64,
    pub kernel_size: u64,
    pub dtb_base: u64,
    pub dtb_size: u64,
    pub uart_base: u64,
    pub gic_dist_base: u64,
    pub gic_redist_base: u64,
    pub cpu_count: u32,
    /// Each CPU's MPIDR affinity, in device-tree order. PSCI needs one to name
    /// a core to power on and the GIC needs one to match a redistributor
    /// frame; a plain counter only happens to work on a single-cluster board.
    /// Fixed storage for the same reason as the memory map: filled from the
    /// device tree before the heap exists.
    pub cpu_affinities: [u64; MAX_CPUS],
    pub cpu_affinity_count: usize,
    pub timer_phys_intid: u32,
    pub timer_virt_intid: u32,
    pub gic_unsupported: bool,
    /// The PCI host bridge's configuration and I/O windows, all zero when the
    /// board has no PCI at all.
    pub pci_ecam_base: u64,
    pub pci_ecam_size: u64,
    pub pci_io_cpu_base: u64,
    pub pci_io_port_base: u64,
    pub pci_io_size: u64,
    /// The memory window the host bridge forwards to its devices. Booting from
    /// a device tree means no firmware has assigned any BAR, so the kernel
    /// hands out addresses from here itself.
    pub pci_mmio_base: u64,
    pub pci_mmio_size: u64,
    /// MMIO base of the PL031 real-time clock, zero when the board has none.
    pub rtc_base: u64,
    /// Fixed storage: this is filled from the device tree in `kernel_entry`,
    /// long before the heap exists, so it cannot be a `Vec`.
    pub memory_regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    pub memory_region_count: usize,
}
