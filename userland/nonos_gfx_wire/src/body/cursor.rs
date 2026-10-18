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

//! CURSOR (0x17): flags u32 (bit 0 visible, bit 1 a new shape in the
//! cursor slot), x i32, y i32 (the pointer's position on the output), hot
//! x, hot y, width, height u16 (at most 64). ENOSYS without HW_CURSOR.
//! MODE_SET (0x15) is width, height, flags u32 (flags 0); RETIRE (0x16)
//! a serial u32: both small enough to read with `bytes` alone.

use crate::bytes::{i32_at, put, u16_at, u32_at};
use crate::errno::{Errno, EINVAL};

pub const CURSOR_LEN: usize = 20;
pub const VISIBLE: u32 = 1 << 0;
pub const NEW_SHAPE: u32 = 1 << 1;
/// The cursor slot holds a 64 x 64 ARGB8888 image.
pub const MAX_SIDE: u16 = 64;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cursor {
    pub flags: u32,
    pub x: i32,
    pub y: i32,
    pub hot_x: u16,
    pub hot_y: u16,
    pub width: u16,
    pub height: u16,
}

impl Cursor {
    pub fn encode(&self) -> [u8; CURSOR_LEN] {
        let mut b = [0u8; CURSOR_LEN];
        put(&mut b, 0, &self.flags.to_le_bytes());
        put(&mut b, 4, &self.x.to_le_bytes());
        put(&mut b, 8, &self.y.to_le_bytes());
        for (i, v) in [self.hot_x, self.hot_y, self.width, self.height].iter().enumerate() {
            put(&mut b, 12 + i * 2, &v.to_le_bytes());
        }
        b
    }

    /// Read CURSOR; a shape past 64 or a hot spot outside it is refused.
    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        let h = |i: usize| u16_at(b, 12 + i * 2).ok_or(EINVAL);
        let c = Cursor {
            flags: u32_at(b, 0).ok_or(EINVAL)?,
            x: i32_at(b, 4).ok_or(EINVAL)?,
            y: i32_at(b, 8).ok_or(EINVAL)?,
            hot_x: h(0)?,
            hot_y: h(1)?,
            width: h(2)?,
            height: h(3)?,
        };
        let fits = c.width <= MAX_SIDE && c.height <= MAX_SIDE;
        let hot = (c.hot_x < c.width.max(1)) && (c.hot_y < c.height.max(1));
        (fits && hot).then_some(c).ok_or(EINVAL)
    }
}
