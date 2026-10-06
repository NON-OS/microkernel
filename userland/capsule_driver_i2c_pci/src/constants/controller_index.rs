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
/// Index of an LPSS I2C function among its platform's controllers (the `n`
/// in the firmware's I2Cn device name), from the PCI device id. Intel
/// firmware names the functions in PCI function order: the contiguous block
/// is I2C0..I2C3 (I2C0..I2C7 on the Broxton family), the pair outside it
/// I2C4/I2C5, and on Tiger Lake-LP and Alder Lake-P a further pair I2C6/I2C7.
/// Lets the driver match the ACPI `_CRS` ResourceSource name against an
/// enumerated PCI function; None means the probe decides alone.
pub fn controller_index(device: u16) -> Option<u8> {
    let block = |first: u16, n: u16| (device >= first && device < first + n).then(|| (device - first) as u8);
    let even = |first: u16| {
        (device >= first && device <= first + 14 && (device - first).is_multiple_of(2))
            .then(|| ((device - first) / 2) as u8)
    };
    let pair = |a: u16, b: u16, base: u8| {
        if device == a {
            Some(base)
        } else if device == b {
            Some(base + 1)
        } else {
            None
        }
    };
    even(0x31AC)
        .or_else(|| even(0x5AAC))
        .or_else(|| even(0x0AAC))
        .or_else(|| even(0x1AAC))
        .or_else(|| block(0x9D60, 6))
        .or_else(|| block(0xA160, 3))
        .or_else(|| block(0xA2E0, 4))
        .or_else(|| block(0xA3E0, 4))
        .or_else(|| block(0xA368, 4))
        .or_else(|| block(0x06E8, 4))
        .or_else(|| block(0x9DE8, 4).or_else(|| pair(0x9DC5, 0x9DC6, 4)))
        .or_else(|| block(0x02E8, 4).or_else(|| pair(0x02C5, 0x02C6, 4)))
        .or_else(|| block(0x4DE8, 4).or_else(|| pair(0x4DC5, 0x4DC6, 4)))
        .or_else(|| block(0x34E8, 4).or_else(|| pair(0x34C5, 0x34C6, 4)))
        .or_else(|| block(0xA0E8, 4).or_else(|| pair(0xA0C5, 0xA0C6, 4)))
        .or_else(|| pair(0xA0D8, 0xA0D9, 6))
        .or_else(|| block(0x43E8, 4))
        .or_else(|| block(0x51E8, 4).or_else(|| pair(0x51C5, 0x51C6, 4)))
        .or_else(|| pair(0x51D8, 0x51D9, 6))
        .or_else(|| block(0x54E8, 4).or_else(|| pair(0x54C5, 0x54C6, 4)))
        .or_else(|| block(0x7ACC, 4).or_else(|| pair(0x7AFC, 0x7AFD, 4)))
        .or_else(|| block(0x7A4C, 4).or_else(|| pair(0x7A7C, 0x7A7D, 4)))
        .or_else(|| block(0x7E78, 4).or_else(|| pair(0x7E50, 0x7E51, 4)))
}
