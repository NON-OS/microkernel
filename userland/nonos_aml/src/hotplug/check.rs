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

//! The device Notify values that ask OSPM to look again (ACPI 6.5, 5.6.6;
//! the values are recalled and unverified here): 0, a bus check, re-read
//! the device and everything below it; 1, a device check, the device has
//! come or gone; 3, an eject request; 4, a device check light, the device
//! alone, nothing below it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Check {
    Bus,
    Device,
    DeviceLight,
    Eject,
}

impl Check {
    /// Whether the devices below the notified one are looked at too.
    pub fn deep(self) -> bool {
        matches!(self, Check::Bus | Check::Device)
    }
}

pub fn check(value: u8) -> Option<Check> {
    match value {
        0 => Some(Check::Bus),
        1 => Some(Check::Device),
        3 => Some(Check::Eject),
        4 => Some(Check::DeviceLight),
        _ => None,
    }
}
