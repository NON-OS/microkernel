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

//! A primary: the surface the compositor draws into and the driver shows.
//! A server holds the current one and, after a MODE_SET, the one it
//! replaced until the compositor sends RETIRE for it.

use super::platform::Region;
use nonos_gfx_wire::body::surface::ARGB8888;
use nonos_gfx_wire::serial::serial;
use nonos_gfx_wire::Surface;

/// The cursor slot: 64 x 64 pixels of four bytes after the frame.
pub const CURSOR_SLOT: usize = 64 * 64 * 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Primary {
    pub region: Region,
    pub handle: u64,
    pub serial: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// 0 when the device has no hardware cursor.
    pub cursor_offset: u32,
}

impl Primary {
    /// The bytes a `w` x `h` primary needs, with a cursor slot or not.
    pub fn len(w: u32, h: u32, cursor: bool) -> Option<(usize, u32, u32)> {
        let stride = w.checked_mul(4)?;
        let frame = (stride as usize).checked_mul(h as usize)?;
        let cursor_offset = if cursor { u32::try_from(frame).ok()? } else { 0 };
        let len = frame.checked_add(if cursor { CURSOR_SLOT } else { 0 })?;
        Some((len, stride, cursor_offset))
    }

    pub fn surface(&self) -> Surface {
        Surface {
            handle: self.handle,
            serial: self.serial,
            width: self.width,
            height: self.height,
            stride: self.stride,
            format: ARGB8888,
            cursor_offset: self.cursor_offset,
        }
    }
}

/// The next serial of driver `pid`: counter `n`, skipping the counter's
/// zero so no serial is ever 0 in its low half.
pub fn next_serial(pid: u32, n: &mut u32) -> u32 {
    *n = (*n + 1) & 0xFFFF;
    if *n == 0 {
        *n = 1;
    }
    serial(pid, *n)
}
