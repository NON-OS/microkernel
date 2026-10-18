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

//! The controller in the device table: a PIIX3 or PIIX4 IDE function with
//! its bus master window and at least one channel whose two windows are
//! listed.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::constants::pci::{
    command_bar, control_bar, BUS_MASTER_BAR, BUS_MASTER_MIN_PORTS, CHANNELS, CLASS_STORAGE,
    COMMAND_MIN_PORTS, CONTROL_MIN_PORTS, INTEL_VENDOR_ID, PIIX3_IDE_DEVICE_ID,
    PIIX4_IDE_DEVICE_ID, SUBCLASS_IDE,
};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    /// Which channels have both windows listed.
    pub channels: [bool; CHANNELS],
}

pub fn find_ide() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..core::cmp::min(n as usize, MAX_DEVICES)].iter().find_map(found)
}

/// The record as this driver takes it, or None for any other device.
pub fn found(r: &DeviceRecord) -> Option<Found> {
    let ids = r.vendor == INTEL_VENDOR_ID
        && (r.device == PIIX3_IDE_DEVICE_ID || r.device == PIIX4_IDE_DEVICE_ID);
    if !ids
        || r.bus_kind != BUS_KIND_PCI
        || r.pci_class != CLASS_STORAGE
        || r.pci_subclass != SUBCLASS_IDE
        || !io(r, BUS_MASTER_BAR, BUS_MASTER_MIN_PORTS)
    {
        return None;
    }
    let channels: [bool; CHANNELS] = core::array::from_fn(|c| {
        io(r, command_bar(c), COMMAND_MIN_PORTS) && io(r, control_bar(c), CONTROL_MIN_PORTS)
    });
    channels.contains(&true).then_some(Found { device_id: r.device_id, channels })
}

fn io(r: &DeviceRecord, bar: u8, min: u16) -> bool {
    let b = r.bars[bar as usize];
    r.bar_count > bar && b.kind == BAR_KIND_PIO && b.size >= min as u64
}
