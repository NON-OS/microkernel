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

//! The serial-bus classes: Intel's SMBus host is the i801 capsule's, which
//! takes only Intel's (capsule_driver_smbus/src/discover.rs); other parts
//! keep their family.

use super::classify_serial_bus::classify_serial_bus;
use super::family::HardwareFamily;

#[test]
fn an_intel_smbus_host_is_the_i801_capsules() {
    assert_eq!(classify_serial_bus(0x05, 0x00, 0x8086), HardwareFamily::SerialSmbus);
    // AMD's FCH host has its own register set; it keeps the old family.
    assert_eq!(classify_serial_bus(0x05, 0x00, 0x1022), HardwareFamily::SerialI2c);
}

#[test]
fn the_usb_hosts_are_unchanged() {
    for (progif, want) in [
        (0x30, HardwareFamily::UsbXhci),
        (0x20, HardwareFamily::UsbEhci),
        (0x10, HardwareFamily::UsbOhci),
        (0x00, HardwareFamily::UsbUhci),
    ] {
        assert_eq!(classify_serial_bus(0x03, progif, 0x8086), want);
    }
    assert_eq!(classify_serial_bus(0x80, 0x00, 0x8086), HardwareFamily::Unknown);
}
