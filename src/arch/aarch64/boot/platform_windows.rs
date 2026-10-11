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

//! The device windows the boot path hands out before any driver runs.

use super::info::BootInfo;

/// Publish the PCI I/O and memory windows and the RTC base from `boot_info`.
pub(super) fn publish(boot_info: &BootInfo) {
    // Port numbers are offsets into the bridge's I/O window on this arch, so
    // the window has to be known before any driver reaches for a port. ECAM is
    // the only way to config space here, and enumeration needs it.
    crate::arch::port_io::set_io_window(
        boot_info.pci_io_cpu_base,
        boot_info.pci_io_port_base,
        boot_info.pci_io_size,
    );
    super::pci_windows::publish(boot_info);
    // Bus addresses, so the I/O window is named by its port base rather than
    // where the CPU reaches it.
    crate::bus::pci::set_windows(
        boot_info.pci_mmio_base,
        boot_info.pci_mmio_size,
        boot_info.pci_io_port_base,
        boot_info.pci_io_size,
    );
    crate::arch::aarch64::rtc::set_base(boot_info.rtc_base);
    if let Some((info, map)) = boot_info.smmu {
        crate::arch::aarch64::iommu::smmu::describe(info, map);
    }
    if let Some((ctrl, map)) = boot_info.msi {
        crate::arch::aarch64::gic::msi::describe(ctrl, map);
    }
}
