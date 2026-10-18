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

use super::classify_display::{classify_display, svga_ports};
use super::classify_network::classify_network;
use super::classify_serial_bus::classify_serial_bus;
use super::classify_storage::classify_storage;
use super::emmc::is_intel_emmc;
use super::family::HardwareFamily;
use super::platform_sd::is_platform_sd_host;
use crate::hardware::broker::{BarKind, DeviceRecord};

/// AHCI 1.3.1, 2.1.11: the ABAR is BAR5.
const AHCI_ABAR_BAR: usize = 5;

pub fn classify_family(
    class: u8,
    subclass: u8,
    progif: u8,
    vendor: u16,
    device: u16,
) -> HardwareFamily {
    match class {
        0x01 => classify_storage(subclass, vendor, device),
        0x02 => classify_network(subclass, vendor, device),
        0x03 => classify_display(vendor),
        0x04 if vendor == 0x8086 && device == 0x2415 => HardwareFamily::AudioAc97,
        0x04 => match subclass {
            0x01 | 0x03 => HardwareFamily::AudioHda,
            _ => HardwareFamily::Unknown,
        },
        0x06 => HardwareFamily::BridgePci,
        0x08 if vendor == 0x80EE && device == 0xCAFE => HardwareFamily::InputVmmdev,
        0x08 if is_intel_emmc(vendor, device, class, subclass) => HardwareFamily::StorageEmmc,
        0x08 => HardwareFamily::SystemPeripheral,
        0x09 if vendor == 0x1af4 && device == 0x1052 => HardwareFamily::InputVirtio,
        0x0c => classify_serial_bus(subclass, progif),
        _ => HardwareFamily::Unknown,
    }
}

/// The family of one listed device. `classify_family` decides from the ids
/// alone; an Intel RAID-mode function it takes for AHCI is kept only when
/// BAR5, where the ABAR lives, is a memory BAR, and VirtualBox's adapter id
/// is SVGA II only with the port window VBoxVGA lacks.
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
