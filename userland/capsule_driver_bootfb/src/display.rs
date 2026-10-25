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

//! The boot framebuffer on the display wire (PLAN G 5): one mode, no mode
//! setting, no cursor; FLUSH copies the shadow in, red and blue swapped
//! for a red-first layout, then fences the write-combined mapping; once a
//! GPU driver has the scanout (`loss`), FLUSH draws nothing, STATE is LOST.

use super::describe;
use super::fb::Fb;
use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ENXIO};
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

pub struct BootFb {
    pub fb: Fb,
    /// Orders the copy's stores before the answer: an sfence on x86.
    pub fence: fn(),
    /// The record; whether it is still held, and letting it go (`loss`).
    pub device_id: u64,
    pub holds: fn(u64) -> bool,
    pub release: fn(u64),
    /// The scanout lost; the compositor told of it.
    pub gone: bool,
    pub told: bool,
}

impl Display for BootFb {
    fn caps(&self) -> Caps {
        describe::caps(&self.fb)
    }

    fn modes(&self, _output: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        *out.first_mut().ok_or(EINVAL)? = describe::mode(&self.fb);
        Ok(1)
    }

    fn state(&mut self, _output: u16) -> Result<State, Errno> {
        Ok(describe::state(&self.fb, self.gone))
    }

    fn set_mode(&mut self, output: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        let all = [Rect { x: 0, y: 0, width: w, height: h }];
        let same = (w, h) == (self.fb.width, self.fb.height);
        same.then_some(()).ok_or(EINVAL).and_then(|_| self.flush(output, &all, shadow))
    }

    fn flush(&mut self, _output: u16, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno> {
        // The serve loop asks before every frame; one lost draws nothing more.
        if self.gone {
            return Err(ENXIO);
        }
        self.fb.copy_in(rects, shadow);
        (self.fence)();
        Ok(())
    }

    fn cursor(&mut self, _output: u16, _c: &Cursor, _slot: &[u8]) -> Result<(), Errno> {
        Err(ENOSYS)
    }
}
