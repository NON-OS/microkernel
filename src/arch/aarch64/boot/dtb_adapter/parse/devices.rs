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

use super::describe;
use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::fdt::find::msi::pci_msi_map;
use crate::arch::fdt::find::pci::find as find_pci;
use crate::arch::fdt::find::rtc::find as find_rtc;
use crate::arch::fdt::find::smmu;
use crate::arch::fdt::find::timer::find as find_timer;
use crate::arch::fdt::find::uart::find as find_uart;
use crate::arch::fdt::Fdt;

/// The devices boot reaches before any driver. With a tree in hand, what it
/// does not describe is absent: PCI and the RTC go to zero instead of
/// keeping virt's addresses for a board that may not have them.
pub(super) fn populate(fdt: &Fdt, info: &mut BootInfo) -> Result<(), &'static [u8]> {
    // No UART we drive keeps the default: a guess at the console costs less
    // than a boot with none.
    if let Some(uart) = find_uart(fdt).map_err(describe)? {
        info.uart = uart;
    }
    super::gic::populate(fdt, info)?;
    let timer = find_timer(fdt).map_err(describe)?;
    let timer = timer.ok_or(&b"the device tree names no architected timer"[..])?;
    info.timer_phys_intid = timer.nonsecure_phys_intid;
    info.timer_virt_intid = timer.virtual_intid;
    crate::arch::aarch64::timer::describe_tree_frequency(timer.clock_frequency);
    let pci = find_pci(fdt).map_err(describe)?.unwrap_or_default();
    info.pci_ecam_base = pci.ecam_base;
    // Today's accessor reads bus 0 at the window base. A window that starts
    // at a later bus waits for the ECAM registry, which takes the start bus.
    info.pci_ecam_size = if pci.bus_start == 0 { pci.ecam_size } else { 0 };
    info.pci_io_cpu_base = pci.io_cpu_base;
    info.pci_io_port_base = pci.io_port_base;
    info.pci_io_size = pci.io_size;
    info.pci_bus_start = pci.bus_start;
    // Before enumeration, which records each function's INTx line from it.
    if let Some(map) = crate::arch::fdt::find::pci::interrupt_map(fdt).map_err(describe)? {
        crate::arch::aarch64::pci_intx::describe(map);
    }
    info.pci_bus_end = pci.bus_end;
    // BARs are assigned from this window by bus address and mapped at the
    // same number, so only a window the bridge does not translate can be
    // used as it is; a translating one is left out until assignment can
    // tell the two apart.
    let identity = pci.mem_cpu_base == pci.mem_bus_base;
    info.pci_mmio_base = if identity { pci.mem_cpu_base } else { 0 };
    info.pci_mmio_size = if identity { pci.mem_size } else { 0 };
    info.rtc_base = find_rtc(fdt).map_err(describe)?.unwrap_or(0);
    // A malformed iommu-map would confine devices to the wrong streams, so
    // it refuses the tree like any other untrustworthy property.
    if let Some(unit) = smmu::find(fdt).map_err(describe)? {
        info.smmu = Some((unit, smmu::pci_iommu_map(fdt).map_err(describe)?));
    }
    // So would a malformed msi-map: a function's messages would carry
    // another's device id, and reach the capsule that holds that one.
    info.msi = pci_msi_map(fdt).map_err(describe)?;
    Ok(())
}
