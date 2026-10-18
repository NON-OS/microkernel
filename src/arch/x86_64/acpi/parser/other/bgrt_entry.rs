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

//! BGRT, the boot graphics resource table: where firmware drew its logo
//! and where the image is, from the table's bytes with no pointer read
//! (`bgrt_load` reads the image; `kernel_proofs` mounts this by path).
//! After the 36-byte header: the version u16 (1), the status u8 (bit 0
//! the logo is on screen now, bits 2:1 the clockwise rotation in quarter
//! turns), the image type u8 (0, a BMP), the image's physical address
//! u64, and its top-left corner's x and y u32. Recalled from ACPI 6.5,
//! 5.2.23, not read: unverified.

pub const BGRT_LEN: usize = 56;
pub const IMAGE_BMP: u8 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bgrt {
    pub displayed: bool,
    /// Quarter turns clockwise the screen is rotated by: 0 to 3.
    pub orientation: u8,
    pub image_type: u8,
    pub address: u64,
    pub x: u32,
    pub y: u32,
}

/// None for a short table, a version other than 1, or no image address.
pub fn bgrt(t: &[u8]) -> Option<Bgrt> {
    let t = t.get(..BGRT_LEN)?;
    let u32_at = |at: usize| u32::from_le_bytes([t[at], t[at + 1], t[at + 2], t[at + 3]]);
    if u16::from_le_bytes([t[36], t[37]]) != 1 {
        return None;
    }
    let address = u64::from(u32_at(40)) | u64::from(u32_at(44)) << 32;
    (address != 0).then_some(Bgrt {
        displayed: t[38] & 1 != 0,
        orientation: t[38] >> 1 & 3,
        image_type: t[39],
        address,
        x: u32_at(48),
        y: u32_at(52),
    })
}
