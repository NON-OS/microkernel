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

//! The VRAM model as a wire display; flush uses the shipping copy.

use super::vram::Vram;
use crate::copy::copy_rects;
use crate::display::{Display, Shadow};
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

impl Display for Vram {
    fn caps(&self) -> Caps {
        self.caps
    }
    fn modes(&self, _o: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        out[..self.modes.len()].copy_from_slice(&self.modes);
        Ok(self.modes.len())
    }
    fn state(&mut self, _o: u16) -> Result<State, Errno> {
        Ok(State {
            generation: 1,
            width: self.size.0,
            height: self.size.1,
            flags: 1,
            ..State::default()
        })
    }
    fn set_mode(&mut self, o: u16, w: u32, h: u32, s: &Shadow) -> Result<(), Errno> {
        if self.fail_set {
            return Err(EINVAL);
        }
        self.size = (w, h);
        self.flush(o, &[Rect { x: 0, y: 0, width: w, height: h }], s)
    }
    fn flush(&mut self, _o: u16, rects: &[Rect], s: &Shadow) -> Result<(), Errno> {
        let len = self.fb.len() * 4;
        let stride = self.size.0 as usize * 4;
        // SAFETY: fb is a live, aligned Vec of len bytes for the call.
        unsafe {
            copy_rects(
                s.bytes,
                s.stride as usize,
                self.fb.as_mut_ptr() as *mut u8,
                len,
                stride,
                rects,
                false,
            )
        };
        Ok(())
    }
    fn cursor(&mut self, _o: u16, c: &Cursor, slot: &[u8]) -> Result<(), Errno> {
        self.cursor = Some((*c, slot.to_vec()));
        Ok(())
    }
}
