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

use alloc::format;
use alloc::string::String;

use crate::arch::x86_64::acpi::devices::i2c::{HidInterrupt, I2cHidDevice, I2cHostName};
use crate::sys::boot_log;

use super::hid_record::controller_slot;

fn text(bytes: &[u8]) -> String {
    bytes.iter().take_while(|&&b| b != 0).map(|&b| if b.is_ascii_graphic() { b as char } else { '?' }).collect()
}

/// One plain line per firmware-declared touchpad on the boot console, and a
/// warning in words when the declaration already shows NONOS cannot drive it.
/// A machine without a touchpad on I2C never reaches here, so it stays quiet.
pub(super) fn report(dev: &I2cHidDevice, hosts: &[I2cHostName]) {
    let irq = match dev.interrupt {
        HidInterrupt::Gpio { pin, level, active_high } => format!(
            "GPIO pin {} on {} ({}, active {})",
            pin,
            text(&dev.gpio_controller),
            if level { "level" } else { "edge" },
            if active_high { "high" } else { "low" }
        ),
        HidInterrupt::Apic { gsi, .. } => format!("APIC GSI {}", gsi),
        HidInterrupt::None => String::from("no interrupt declared"),
    };
    let addr = if dev.slave_addr != 0 {
        format!("0x{:02x}", dev.slave_addr)
    } else {
        String::from("set at run time")
    };
    boot_log::info(&format!(
        "touchpad {} on I2C bus {} addr {} {} kHz, {}, HID descriptor at 0x{:04x}{}",
        text(&dev.hid),
        text(&dev.controller),
        addr,
        dev.speed_hz / 1000,
        irq,
        dev.hid_desc_reg,
        if dev.desc_reg_from_dsm { "" } else { " (default, will probe)" }
    ));
    let known_bus = controller_slot(dev.controller) != 0
        || dev.controller == [0; 4]
        || hosts.iter().any(|h| h.name == dev.controller);
    if dev.ten_bit {
        boot_log::warn(&format!(
            "touchpad {} uses 10-bit I2C addressing, which NONOS does not support: no touchpad pointer",
            text(&dev.hid)
        ));
    } else if !known_bus {
        boot_log::warn(&format!(
            "touchpad {} sits on I2C controller {}, which NONOS cannot identify; it will be found only if a supported controller answers it",
            text(&dev.hid),
            text(&dev.controller)
        ));
    }
}
