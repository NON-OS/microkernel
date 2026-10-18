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

use super::hold_platform::*;
use super::hold_rule::Hold;

const ACPI: u8 = BUS_ACPI;
const HOST: Hold = Hold::ForService(I2C_HOST_SERVICE);
const PS2: Hold = Hold::ForService(PS2_SERVICE);

#[test]
fn an_i2c_host_is_held_for_the_i2c_host_driver_and_its_gpio_bank_for_either() {
    assert_eq!(hold_for_platform(ACPI, SERIAL, SERIAL_BUS, 0, 0), Some(HOST));
    let bank = Hold::ForEither(GPIO_SERVICE, I2C_HOST_SERVICE);
    for uid in [0u16, 1, 2, 0xFFFF] {
        assert_eq!(hold_for_platform(ACPI, GPIO_CTRL, 0, 0, uid), Some(bank), "{uid}");
    }
    assert_eq!((I2C_HOST_SERVICE, GPIO_SERVICE), ("driver.i2c_pci0", "driver.gpio0"));
}

#[test]
fn the_ps2_ports_are_held_for_the_ps2_driver_and_nothing_else_of_their_class() {
    assert_eq!(hold_for_platform(ACPI, INPUT, 0, PNP, PS2_KBD), Some(PS2));
    assert_eq!(hold_for_platform(ACPI, INPUT, 0, PNP, PS2_AUX), Some(PS2));
    assert_eq!(PS2_SERVICE, "driver.ps2_kbd0");
    for (vendor, device) in [(PNP, 0x0305), (0x0002, PS2_KBD), (0, 0)] {
        let hold = hold_for_platform(ACPI, INPUT, 0, vendor, device);
        assert_eq!(hold, Some(Hold::Refused), "{vendor:#x}:{device:#x}");
    }
}

#[test]
fn every_other_published_record_is_refused_until_named() {
    // A serial record without the serial-bus class is a UART, not a host.
    assert_eq!(hold_for_platform(ACPI, SERIAL, 0x07, 0, 0), Some(Hold::Refused));
    // An I2C-HID device is read from the list; nothing claims it.
    assert_eq!(hold_for_platform(ACPI, 0x0041, 0, 0x2C, 0x20), Some(Hold::Refused));
    for class in [0u32, 0x0010, 0x0020, 0x0050, 0x0070, 0xFFFF] {
        assert_eq!(hold_for_platform(ACPI, class, 0, 0, 0), Some(Hold::Refused), "{class:#x}");
    }
    // Every device-tree record, until C-13's are named.
    for class in [SERIAL, INPUT, GPIO_CTRL, 0] {
        let hold = hold_for_platform(BUS_DEVICE_TREE, class, SERIAL_BUS, PNP, PS2_KBD);
        assert_eq!(hold, Some(Hold::Refused));
    }
}

#[test]
fn the_buses_other_rules_key_are_left_to_them() {
    // PCI (1) and firmware (5) are keyed elsewhere; the virtual bus (3) has
    // its own rule here (`virt_tests`).
    for bus in [0u8, 1, 5, 6, 0xFF] {
        assert_eq!(hold_for_platform(bus, SERIAL, SERIAL_BUS, PNP, PS2_KBD), None, "{bus}");
    }
}
