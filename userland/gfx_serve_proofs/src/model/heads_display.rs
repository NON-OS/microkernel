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

//! The heads model as a wire display; flush uses the shipping copy.

use super::heads::Heads;
use crate::copy::copy_rects;
use crate::display::{Display, Shadow};
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

impl Display for Heads {
    fn caps(&self) -> Caps {
        self.caps
    }
    fn modes(&self, o: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        let (width, height) = self.heads[o as usize].size;
        out[0] = Mode { width, height, flags: 0b111 };
        Ok(1)
    }
    fn state(&mut self, o: u16) -> Result<State, Errno> {
        let h = &mut self.heads[o as usize];
        let lost = if std::mem::take(&mut h.lost) { LOST } else { 0 };
        let flags = if h.dark { 0 } else { SCANNING_OUT | lost };
        let (width, height) = h.size;
        Ok(State { generation: 1, width, height, flags, ..State::default() })
    }
    fn set_mode(&mut self, o: u16, w: u32, h: u32, s: &Shadow) -> Result<(), Errno> {
        if self.fail_set {
            return Err(EINVAL);
        }
        let head = &mut self.heads[o as usize];
        (head.size, head.sets) = ((w, h), head.sets + 1);
        self.flush(o, &[Rect { x: 0, y: 0, width: w, height: h }], s)
    }
    fn flush(&mut self, o: u16, rects: &[Rect], s: &Shadow) -> Result<(), Errno> {
        let head = &mut self.heads[o as usize];
        let (len, stride) = (head.fb.len() * 4, head.size.0 as usize * 4);
        let fb = head.fb.as_mut_ptr() as *mut u8;
        // SAFETY: fb is this head's live, aligned Vec of len bytes.
        unsafe { copy_rects(s.bytes, s.stride as usize, fb, len, stride, rects, false) };
        Ok(())
    }
    fn cursor(&mut self, _o: u16, _c: &Cursor, _slot: &[u8]) -> Result<(), Errno> {
        Err(ENOSYS)
    }
}
