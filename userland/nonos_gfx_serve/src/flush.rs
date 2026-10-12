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

//! FLUSH and CURSOR: both work on the current primary. A flush names its
//! serial, and a stale one is ESTALE so a compositor that missed a mode
//! change repaints; a rectangle outside the primary is EINVAL. The cursor
//! is shown from the primary's cursor slot, on devices that have one.

use nonos_gfx_wire::body::caps::HW_CURSOR;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ESTALE};
use nonos_gfx_wire::{Cursor, Errno, Flush};

use super::display::Display;
use super::platform::Platform;
use super::primary::CURSOR_SLOT;
use super::server::Server;

impl<D: Display, P: Platform> Server<D, P> {
    pub(super) fn flush(&mut self, output: u16, f: &Flush) -> Result<(), Errno> {
        let cur = self.current.filter(|c| c.serial == f.serial).ok_or(ESTALE)?;
        if !f.as_slice().iter().all(|r| r.inside(cur.width, cur.height)) {
            return Err(EINVAL);
        }
        self.with_shadow(cur, |d, s| d.flush(output, f.as_slice(), s))
    }

    pub(super) fn cursor(&mut self, output: u16, c: &Cursor) -> Result<(), Errno> {
        let cur = self.current.ok_or(EINVAL)?;
        if !self.display.caps().has(HW_CURSOR) || cur.cursor_offset == 0 {
            return Err(ENOSYS);
        }
        let at = cur.cursor_offset as usize;
        let slot = &self.platform.bytes(cur.region)[at..at + CURSOR_SLOT];
        let mut copy = [0u8; CURSOR_SLOT];
        copy.copy_from_slice(slot);
        self.display.cursor(output, c, &copy)
    }
}
