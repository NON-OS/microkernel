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

//! Devices with a pointer that offer report mode: the report keyboard with
//! a pointer Report and a boot mouse added, and a mouse alone.

use super::db::Db;
use super::devices::{NOTIFY, READ, WWR};
use super::report_devices::report_keyboard_with;

/// In the keyboard with a pointer: the pointer Report's value, and the
/// boot mouse report's.
pub const POINTER_IN: u16 = 23;
pub const BOOT_MOUSE: u16 = 27;

/// The report keyboard, then a Report with Reference `pointer`, then a
/// boot mouse input report, all in its one HID service.
pub fn keyboard_pointer(map: &[u8], pointer: &[u8]) -> Db {
    report_keyboard_with(map, [&[1, 1], &[2, 1], &[1, 2]])
        .char(READ | NOTIFY, 0x2a4d, &[0; 4])
        .desc(0x2908, pointer)
        .desc(0x2902, &[0, 0])
        .char(READ | NOTIFY, 0x2a33, &[0; 3])
        .desc(0x2902, &[0, 0])
}

/// In the mouse alone: Protocol Mode, the boot mouse report, and the
/// Report.
pub const MOUSE_MODE: u16 = 3;
pub const MOUSE_BOOT: u16 = 7;
pub const MOUSE_IN: u16 = 10;

/// A mouse with `map`, and its one Report with Reference `reference`.
pub fn mouse_alone(map: &[u8], reference: &[u8]) -> Db {
    Db::default()
        .service(0x1812)
        .char(READ | WWR, 0x2a4e, &[1])
        .char(READ, 0x2a4b, map)
        .char(READ | NOTIFY, 0x2a33, &[0; 3])
        .desc(0x2902, &[0, 0])
        .char(READ | NOTIFY, 0x2a4d, &[0; 6])
        .desc(0x2908, reference)
        .desc(0x2902, &[0, 0])
}
