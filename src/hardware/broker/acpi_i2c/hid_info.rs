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

//! The interrupt and descriptor facts of an I2C-HID record's bar 0.

use crate::arch::firmware_table::devices::GpioController;
use crate::arch::firmware_table::devices::{HidInterrupt, I2cHidDevice};

use super::slots::gpio_community_slot;

/// Interrupt and descriptor facts carried in the I2C-HID record's bar 0
/// `flags` byte. The userland i2c drivers mirror these values.
pub(super) const HID_INFO_GPIO: u8 = 1 << 0;
pub(super) const HID_INFO_APIC: u8 = 1 << 1;
pub(super) const HID_INFO_LEVEL: u8 = 1 << 2;
pub(super) const HID_INFO_ACTIVE_HIGH: u8 = 1 << 3;
pub(super) const HID_INFO_DESC_FROM_DSM: u8 = 1 << 4;
pub(super) const HID_INFO_TEN_BIT: u8 = 1 << 5;

/// The HID_INFO_* bits, the interrupt source and the GPIO community slot.
pub(super) fn hid_info(dev: &I2cHidDevice, gpio: &[GpioController]) -> (u8, u32, u8) {
    let mut info = 0u8;
    let mut source = 0u32;
    let mut community = 0u8;
    let (level, active_high) = match dev.interrupt {
        HidInterrupt::Gpio { pin, level, active_high } => {
            info |= HID_INFO_GPIO;
            source = u32::from(pin);
            community = gpio_community_slot(dev.gpio_controller, gpio);
            (level, active_high)
        }
        HidInterrupt::Apic { gsi, level, active_high } => {
            info |= HID_INFO_APIC;
            source = gsi;
            (level, active_high)
        }
        HidInterrupt::None => (false, false),
    };
    if level {
        info |= HID_INFO_LEVEL;
    }
    if active_high {
        info |= HID_INFO_ACTIVE_HIGH;
    }
    if dev.desc_reg_from_dsm {
        info |= HID_INFO_DESC_FROM_DSM;
    }
    if dev.ten_bit {
        info |= HID_INFO_TEN_BIT;
    }
    (info, source, community)
}
