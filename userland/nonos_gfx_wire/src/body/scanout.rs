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

//! SCANOUT (0x18), 44 bytes: the client surface's handle u64, the serial
//! of the primary the band is read from u32, the surface's width, height,
//! stride and format u32, then the band's rectangle on the output (x, y,
//! width, height u32). The driver shows the surface whole on the output's
//! plane and the band's pixels from the primary on a plane over it, both
//! from the same vertical blank; it answers once they are live. FLUSH of
//! the primary afterwards shows the primary again and lets the surface go.

use super::rect::Rect;
use super::surface::ARGB8888;
use crate::bytes::{put, u32_at, u64_at};
use crate::errno::{Errno, EINVAL};

pub const SCANOUT_LEN: usize = 44;
/// The band's plane holds at most this much: four pages at a 512-byte
/// stride.
pub const BAND_MAX_W: u32 = 128;
pub const BAND_MAX_H: u32 = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Scanout {
    pub handle: u64,
    pub serial: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub band: Rect,
}

impl Scanout {
    pub fn encode(&self) -> [u8; SCANOUT_LEN] {
        let mut b = [0u8; SCANOUT_LEN];
        put(&mut b, 0, &self.handle.to_le_bytes());
        let r = self.band;
        let words = [self.serial, self.width, self.height, self.stride, self.format];
        let band = [r.x, r.y, r.width, r.height];
        for (i, v) in words.iter().chain(band.iter()).enumerate() {
            put(&mut b, 8 + i * 4, &v.to_le_bytes());
        }
        b
    }

    /// Read a SCANOUT body: no handle, an empty surface, a stride shorter
    /// than a row, a format other than ARGB8888, or a band that is empty,
    /// larger than its plane or off the surface is refused.
    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        let w = |i: usize| u32_at(b, 8 + i * 4).ok_or(EINVAL);
        let s = Scanout {
            handle: u64_at(b, 0).ok_or(EINVAL)?,
            serial: w(0)?,
            width: w(1)?,
            height: w(2)?,
            stride: w(3)?,
            format: w(4)?,
            band: Rect { x: w(5)?, y: w(6)?, width: w(7)?, height: w(8)? },
        };
        let row = s.width as u64 * 4;
        let shape = s.handle != 0 && s.width != 0 && s.height != 0 && s.stride as u64 >= row;
        let band = s.band.inside(s.width, s.height)
            && s.band.width <= BAND_MAX_W
            && s.band.height <= BAND_MAX_H;
        (shape && s.format == ARGB8888 && band).then_some(s).ok_or(EINVAL)
    }
}
