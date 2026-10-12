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

//! The engine as a `nonos_gfx_serve::Display`, one wire output per taken
//! over pipe. FLUSH copies the rectangles into the output's back surface,
//! flips to it at the vertical blank, then copies them into the new back
//! as well, so both surfaces hold the whole picture and the next FLUSH
//! needs only its own rectangles: nothing is drawn into a surface the
//! engine is scanning out, so nothing tears.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::{ENOSYS, ENXIO, ETIMEDOUT};
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

use super::copy::copy;
use super::describe;
use super::frames::Frames;
use super::intel::{Intel, FLIP_US};
use super::modes::modes;
use crate::engine::access::{Clock, Mmio, Until};

impl<M: Mmio, F: Frames, C: Clock> Display for Intel<M, F, C> {
    fn caps(&self) -> Caps {
        describe::caps(self)
    }

    fn modes(&self, output: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        Ok(modes(self.head(output)?, out))
    }

    fn state(&mut self, output: u16) -> Result<State, Errno> {
        let flags = self.health(output)?;
        let h = self.head(output)?;
        let (width, height, generation) = (h.scan.width, h.scan.height, h.generation);
        Ok(State { generation, width, height, flags, ..State::default() })
    }

    fn set_mode(&mut self, output: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        self.change_mode(output, w, h, shadow)
    }

    fn flush(&mut self, output: u16, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno> {
        let Intel { m, clock, heads, .. } = self;
        let h = heads.get_mut(output as usize).and_then(|h| h.as_mut()).ok_or(ENXIO)?;
        let (size, stride) = ((h.scan.width, h.scan.height), h.scan.stride);
        copy(&mut h.frames, h.scan.back(), size, stride, rects, shadow)?;
        h.frames.fence();
        let until = Until::new(clock, FLIP_US);
        h.scan.present(m, &until).map_err(|_| ETIMEDOUT)?;
        // Checked whole above, so the second copy cannot be refused.
        copy(&mut h.frames, h.scan.back(), size, stride, rects, shadow)
    }

    fn cursor(&mut self, output: u16, _c: &Cursor, _slot: &[u8]) -> Result<(), Errno> {
        self.head(output).and(Err(ENOSYS))
    }
}
