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

use super::classify::classify_family;
use super::classify_display::svga_ports;
use super::family::HardwareFamily;
use super::platform_sd::is_platform_sd_host;
use crate::hardware::broker::{BarKind, DeviceRecord};

/// AHCI 1.3.1, 2.1.11: the ABAR is BAR5.
const AHCI_ABAR_BAR: usize = 5;

/// The family of one listed device. `classify_family` decides from the ids
/// alone; an Intel RAID-mode function it takes for AHCI is kept only when
/// BAR5, where the ABAR lives, is a memory BAR, and VirtualBox's adapter id
/// is SVGA II only with the port window VBoxVGA lacks. An SD host the ACPI
/// tables publish is served by the SD host capsule as a PCI one is.
pub fn classify_device(rec: &DeviceRecord) -> HardwareFamily {
    if is_platform_sd_host(rec.bus_kind, rec.pci_class, rec.pci_subclass) {
        return HardwareFamily::StorageSdhci;
    }
    let family =
        classify_family(rec.pci_class, rec.pci_subclass, rec.pci_progif, rec.vendor, rec.device);
    if family == HardwareFamily::StorageAhci && rec.pci_subclass == 0x04 && !abar_mapped(rec) {
        return HardwareFamily::Unknown;
    }
    if family == HardwareFamily::DisplayVmwareSvga && !svga_ports(rec) {
        return HardwareFamily::Unknown;
    }
    family
}

fn abar_mapped(rec: &DeviceRecord) -> bool {
    let bar = rec.bars[AHCI_ABAR_BAR];
    rec.bar_count as usize > AHCI_ABAR_BAR && bar.kind == BarKind::Mmio as u8 && bar.size != 0
}
