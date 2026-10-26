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

//! The touchpads the capsule finds are the I2C-HID records the broker lists
//! (`find_touchpad_addrs`, by path): an ACPI record of that class, its slave
//! address in the vendor word and its descriptor register in the device
//! word. Any other record names no touchpad, and a failed list names none.

use nonos_libc::{list_devices, DeviceRecord, BUS_KIND_ACPI, BUS_KIND_PCI};

use super::acpi_touchpad::AcpiTouchpad;
use super::defs::CLASS_I2C_HID;
use super::find_touchpad_addrs::find_touchpad_addrs;

fn record(bus: u8, class: u32, addr: u16, reg: u16) -> DeviceRecord {
    let mut r = DeviceRecord::empty();
    (r.bus_kind, r.class, r.vendor, r.device) = (bus, class, addr, reg);
    r
}

#[test]
fn each_acpi_i2c_hid_record_is_a_touchpad_and_nothing_else_is() {
    list_devices(Ok(vec![
        record(BUS_KIND_ACPI, CLASS_I2C_HID, 0x2C, 0x20),
        record(BUS_KIND_PCI, CLASS_I2C_HID, 0x15, 0x01),
        record(BUS_KIND_ACPI, 0x0060, 0x15, 0x01),
        record(BUS_KIND_ACPI, CLASS_I2C_HID, 0x95, 0x01),
    ]));
    let mut out = [AcpiTouchpad::default(); 4];
    assert_eq!(find_touchpad_addrs(&mut out), 2);
    let found: Vec<(u8, u16)> = out[..2].iter().map(|t| (t.addr, t.desc_reg)).collect();
    assert_eq!(found, [(0x2C, 0x20), (0x15, 0x01)], "the address is seven bits");
    list_devices(Err(-14));
    assert_eq!(find_touchpad_addrs(&mut out), 0);
}

/// The broker publishes I2C-HID under the class id its ABI names.
#[test]
fn the_capsules_class_is_the_brokers() {
    let abi = include_str!("../../../../abi/driver_broker_abi.md");
    assert!(abi.contains(&format!("CLASS_I2C_HID        = {CLASS_I2C_HID:#06x}")));
}
