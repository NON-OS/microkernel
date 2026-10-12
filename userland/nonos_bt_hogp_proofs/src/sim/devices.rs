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

//! The devices the proofs pair with, laid out as HOGP 1.0 and HIDS 1.0
//! describe a HID device's database.

use super::db::Db;

pub const READ: u8 = 0x02;
pub const WWR: u8 = 0x04;
pub const WRITE: u8 = 0x08;
pub const NOTIFY: u8 = 0x10;

/// A vendor's own 128-bit characteristic, outside the Bluetooth base.
const VENDOR: [u8; 16] = [
    0x9e, 0xca, 0xdc, 0x24, 0x0e, 0xe5, 0xa9, 0xe0, 0x93, 0xf3, 0xa3, 0xb5, 0x01, 0x00, 0x40, 0x6e,
];

/// A keyboard with a pointing stick: GAP, one HID service with Protocol
/// Mode, a vendor characteristic with a 128-bit UUID, the boot keyboard
/// input and output reports and the boot mouse input report, then a
/// battery service. The mouse report has a Report
/// Reference before its CCCD, so the CCCD has to be looked for.
pub fn keyboard_mouse() -> Db {
    Db::default()
        .service(0x1800)
        .char(READ, 0x2a00, b"keys")
        .service(0x1812)
        .char(READ | WWR, 0x2a4e, &[1])
        .char_field(READ | NOTIFY, &VENDOR, &[0; 4])
        .desc(0x2902, &[0, 0])
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .char(READ | WRITE | WWR, 0x2a32, &[0])
        .char(READ | NOTIFY, 0x2a33, &[0; 3])
        .desc(0x2908, &[0, 1])
        .desc(0x2902, &[0, 0])
        .char(READ, 0x2a4b, &[0x05, 0x01])
        .service(0x180f)
        .char(READ | NOTIFY, 0x2a19, &[90])
        .desc(0x2902, &[0, 0])
}

/// A combined device with the keyboard and the mouse in two HID services.
pub fn combo() -> Db {
    Db::default()
        .service(0x1812)
        .char(READ | WWR, 0x2a4e, &[1])
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .service(0x1812)
        .char(READ | WWR, 0x2a4e, &[1])
        .char(READ | NOTIFY, 0x2a33, &[0; 3])
        .desc(0x2902, &[0, 0])
}
