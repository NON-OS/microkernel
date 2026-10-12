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

use super::limits::MAX_CPUS;
use super::reserved::Reserved;
use super::types::BootInfo;
use super::virt_ram::{memory_map, RAM_BASE, RAM_SIZE};
use super::virt_uart::VIRT_UART;

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            ram_base: RAM_BASE,
            ram_size: RAM_SIZE,
            kernel_base: 0x4000_0000,
            kernel_size: 0x0020_0000,
            dtb_base: 0,
            dtb_size: 0,
            uart: VIRT_UART,
            // virt puts the distributor at the same place for either GIC, the
            // GICv3 redistributors at 0x080a_0000 and the GICv2 CPU interface
            // at 0x0801_0000. Which one is there is the CPU's to say.
            gic_version: None,
            gic_dist_base: 0x0800_0000,
            gic_redist_base: 0x080A_0000,
            gic_cpu_base: 0x0801_0000,
            cpu_count: 1,
            cpu_affinities: [0; MAX_CPUS],
            cpu_affinity_count: 0,
            cpu_release: [None; MAX_CPUS],
            // Every default here describes the QEMU virt board, and the timer is
            // no exception: PPI 14 and 11 are the non-secure physical and virtual
            // timers, which are INTID 30 and 27 once the 16 SGIs are counted. A
            // device tree that names them overrides this.
            timer_phys_intid: 30,
            timer_virt_intid: 27,
            // The virt board's PCIe host bridge. Config space is the high ECAM
            // window at 256 GiB, which is where the board puts it whenever the
            // CPU can address that far. The 16 MiB window at 0x3f00_0000 is the
            // older placement and reads there return nothing, which surfaces as
            // an external abort rather than a translation fault, since the page
            // tables are perfectly happy to map an address no device answers.
            // The I/O window at 0x3eff_0000 starts at port 0: port N is +N.
            pci_ecam_base: 0x40_1000_0000,
            pci_ecam_size: 0x1000_0000,
            pci_io_cpu_base: 0x3eff_0000,
            pci_io_port_base: 0,
            pci_io_size: 0x0001_0000,
            pci_bus_start: 0,
            pci_bus_end: 0xff,
            pci_mmio_base: 0x1000_0000,
            pci_mmio_size: 0x2eff_0000,
            rtc_base: 0x0901_0000, // PL031, the board's real-time clock
            smmu: None,            // only `iommu=smmuv3` adds one, and a tree says so
            msi: None,             // with no tree there is no msi-map to bind through
            memory_regions: memory_map(),
            memory_region_count: 1,
            reserved: Reserved::EMPTY,
            initrd: None,
        }
    }
}
