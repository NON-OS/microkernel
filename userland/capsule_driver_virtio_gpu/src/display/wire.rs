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
//! virtio-gpu as a `nonos_gfx_serve::Display`. The server has checked the
//! output against CAPS and every rectangle against the primary, which is
//! the size of the surface the head shows.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::{ENOSYS, ENXIO};
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

use super::gpu::Gpu;
use super::plug::Plug;
use super::{describe, modes};

impl Display for Gpu {
    fn caps(&self) -> Caps {
        describe::caps(self)
    }

    fn modes(&self, output: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        self.head(output).ok_or(ENXIO)?;
        Ok(modes::modes(self, output, out))
    }

    fn state(&mut self, output: u16) -> Result<State, Errno> {
        // The compositor's 500 ms poll: limbo is tried on it, not only when
        // a mode is set, so a host back again gets its pages freed soon.
        self.retry_limbo();
        let _ = self.events();
        describe::state(self, output)
    }

    fn set_mode(&mut self, output: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        describe::live(self)?;
        self.mode_set(output, w, h, shadow)
    }

    fn flush(&mut self, output: u16, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno> {
        describe::live(self)?;
        let h = self.head(output).ok_or(ENXIO)?;
        // Off, or back but not yet shown again: nothing is on the screen to
        // update, and the mode set that shows it again loads the whole.
        if h.plug != Plug::In {
            return Ok(());
        }
        self.load(&h.shown, rects, shadow)?;
        self.flush_rects(&h.shown, rects)
    }

    fn cursor(&mut self, _output: u16, _c: &Cursor, _slot: &[u8]) -> Result<(), Errno> {
        Err(ENOSYS)
    }
}
