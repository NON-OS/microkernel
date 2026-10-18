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

//! The device half a driver writes (section 12's `Display` trait), and
//! the shadow it is handed: the primary's memory, which the compositor
//! draws and FLUSH copies from.

use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

/// A primary's memory as the driver sees it: `bytes` hold `height` rows
/// of `stride` bytes, ARGB8888, then the cursor slot when there is one.
pub struct Shadow<'a> {
    pub bytes: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// Where the 64 x 64 cursor image starts in `bytes`; 0 for none.
    pub cursor_offset: u32,
}

pub trait Display {
    fn caps(&self) -> Caps;
    fn modes(&self, output: u16, out: &mut [Mode]) -> Result<usize, Errno>;
    /// Device events are read here; the server fills in the serial.
    fn state(&mut self, output: u16) -> Result<State, Errno>;
    /// Program `w` x `h` and show `shadow` on it before returning.
    fn set_mode(&mut self, output: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno>;
    /// Make `rects` of `shadow` visible before returning.
    fn flush(&mut self, output: u16, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno>;
    fn cursor(&mut self, output: u16, c: &Cursor, slot: &[u8]) -> Result<(), Errno>;
}
