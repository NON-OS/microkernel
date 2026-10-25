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

use crate::arch::firmware_table::devices::GpioController;
use crate::arch::firmware_table::devices::I2cHidDevice;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, BusKind, DeviceRecord};

use super::hid_info::hid_info;
use super::slots::controller_slot;

/// Build an ACPI-bus DeviceRecord for a firmware-declared I2C-HID device.
///
/// * `vendor`: 7-bit slave address, zero when the firmware fills it at run
///   time (the record then still names the controller the pad sits on).
/// * `device`: HID descriptor register (`_DSM` function 1, else 0x0001).
/// * `pci_progif`: the named host controller "I2Cn" as n+1, zero otherwise.
/// * `irq_pin`: GPIO community of a GpioInt as `_UID`+1, zero when unknown.
/// * `irq_source`: the GpioInt pin, or the GSI of an Interrupt resource.
/// * bar 0 (kind None, never a window; `bar_count` stays zero): `base` is
///   `host`, the device_id of the platform host or PCI LPSS function the
///   ResourceSource resolved to, else u64::MAX (no physical address: every
///   MkDeviceList caller reads it), `size` the controller NameSeg as a
///   little-endian u32,
///   `aux` the ConnectionSpeed in Hz, `flags` the HID_INFO_* bits.
pub(super) fn hid_record(dev: &I2cHidDevice, gpio: &[GpioController], host: u64) -> DeviceRecord {
    let (info, source, community) = hid_info(dev, gpio);
    let mut bars = [Bar::empty(); 6];
    bars[0] = Bar {
        base: host,
        size: u64::from(u32::from_le_bytes(dev.controller)),
        kind: BarKind::None as u8,
        flags: info,
        aux: dev.speed_hz,
        _pad: [0; 2],
    };
    DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::I2C_HID,
        vendor: u16::from(dev.slave_addr),
        device: dev.hid_desc_reg,
        pci_progif: controller_slot(dev.controller),
        irq_pin: community,
        irq_source: source,
        bars,
        ..DeviceRecord::empty()
    }
}
