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

//! SURFACE (0x13), and MODE_SET's answer: 32 bytes. The primary's shared
//! surface handle u64, serial, width, height, stride, format (ARGB8888),
//! and the cursor slot's byte offset in it (0 without HW_CURSOR).

use crate::bytes::{put, u32_at, u64_at};
use crate::errno::{Errno, EINVAL};

pub const SURFACE_LEN: usize = 32;
/// The one pixel format of the wire: 32 bits, B G R A in memory order.
pub const ARGB8888: u32 = 1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Surface {
    pub handle: u64,
    pub serial: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub cursor_offset: u32,
}

impl Surface {
    pub fn encode(&self) -> [u8; SURFACE_LEN] {
        let mut b = [0u8; SURFACE_LEN];
        put(&mut b, 0, &self.handle.to_le_bytes());
        let f = [self.serial, self.width, self.height, self.stride, self.format];
        for (i, v) in f.iter().chain([self.cursor_offset].iter()).enumerate() {
            put(&mut b, 8 + i * 4, &v.to_le_bytes());
        }
        b
    }

    /// Read a SURFACE body; a stride shorter than a row, an unknown
    /// format or an empty picture is refused.
    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        let f = |i: usize| u32_at(b, 8 + i * 4).ok_or(EINVAL);
        let s = Surface {
            handle: u64_at(b, 0).ok_or(EINVAL)?,
            serial: f(0)?,
            width: f(1)?,
            height: f(2)?,
            stride: f(3)?,
            format: f(4)?,
            cursor_offset: f(5)?,
        };
        let row = s.width as u64 * 4;
        if s.width == 0 || s.height == 0 || (s.stride as u64) < row || s.format != ARGB8888 {
            return Err(EINVAL);
        }
        Ok(s)
    }
}
