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

use alloc::vec::Vec;

use super::types::PciDevice;
use super::window::BusWindow;
use crate::arch::x86_64::acpi::data::PcieSegment;
use crate::arch::x86_64::acpi::parser;

/// Every function in every MCFG segment and bus range. Buses are walked by
/// number across the whole declared range rather than by following bridges,
/// so a device behind a root port (Wi-Fi and NVMe sit on buses 1..n on Intel
/// laptops) is found whether or not the walk visits its bridge first.
pub fn enumerate_pci_devices() -> Vec<PciDevice> {
    let mut devices = Vec::new();
    for seg in parser::pcie_segments() {
        for bus in seg.start_bus..=seg.end_bus {
            enumerate_bus(&seg, bus, &mut devices);
        }
    }
    devices
}

fn enumerate_bus(seg: &PcieSegment, bus: u8, devices: &mut Vec<PciDevice>) {
    let Some(win) = BusWindow::map(seg, bus) else {
        return;
    };
    for device in 0..32u8 {
        if let Some(dev) = probe_device(&win, seg.segment, bus, device, 0) {
            let is_multifunction = win.read8(device, 0, 0x0E) & 0x80 != 0;
            devices.push(dev);
            if is_multifunction {
                for function in 1..8u8 {
                    if let Some(dev) = probe_device(&win, seg.segment, bus, device, function) {
                        devices.push(dev);
                    }
                }
            }
        }
    }
}

fn probe_device(
    win: &BusWindow,
    segment: u16,
    bus: u8,
    device: u8,
    function: u8,
) -> Option<PciDevice> {
    let vendor_id = win.read16(device, function, 0);
    if vendor_id == 0xFFFF {
        return None;
    }
    Some(PciDevice {
        segment,
        bus,
        device,
        function,
        vendor_id,
        device_id: win.read16(device, function, 2),
        class: win.read8(device, function, 0x0B),
        subclass: win.read8(device, function, 0x0A),
    })
}
