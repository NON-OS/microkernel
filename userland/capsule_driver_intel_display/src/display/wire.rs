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

//! The engine as a `nonos_gfx_serve::Display`. FLUSH copies the rectangles
//! into the back surface, flips to it at the vertical blank, then copies
//! them into the new back as well, so both surfaces hold the whole picture
//! and the next FLUSH needs only its own rectangles: nothing is drawn into
//! a surface the engine is scanning out, so nothing tears.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::{ENOSYS, ETIMEDOUT};
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

use super::copy::copy;
use super::describe;
use super::frames::Frames;
use super::intel::{Intel, FLIP_US};
use crate::engine::access::{Clock, Mmio, Until};

impl<M: Mmio, F: Frames, C: Clock> Display for Intel<M, F, C> {
    fn caps(&self) -> Caps {
        describe::caps(self)
    }

    fn modes(&self, _output: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        Ok(describe::modes(self, out))
    }

    fn state(&mut self, _output: u16) -> Result<State, Errno> {
        let flags = self.health();
        let (width, height, generation) = (self.scan.width, self.scan.height, self.generation);
        Ok(State { generation, width, height, flags, ..State::default() })
    }

    fn set_mode(&mut self, _output: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        self.change_mode(w, h, shadow)
    }

    fn flush(&mut self, _output: u16, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno> {
        let size = (self.scan.width, self.scan.height);
        let stride = self.scan.stride;
        copy(&mut self.frames, self.scan.back(), size, stride, rects, shadow)?;
        self.frames.fence();
        let until = Until::new(&self.clock, FLIP_US);
        self.scan.present(&mut self.m, &until).map_err(|_| ETIMEDOUT)?;
        // Checked whole above, so the second copy cannot be refused.
        copy(&mut self.frames, self.scan.back(), size, stride, rects, shadow)
    }

    fn cursor(&mut self, _output: u16, _c: &Cursor, _slot: &[u8]) -> Result<(), Errno> {
        Err(ENOSYS)
    }
}
