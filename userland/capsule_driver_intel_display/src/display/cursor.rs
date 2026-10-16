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

//! CURSOR on a taken-over output (section 12; the cursor plane as
//! `engine::cursor` drives it): a new shape is copied from the primary's
//! cursor slot into the cursor's own memory, mapped once at the bottom of
//! the output's GGTT window; then the plane is shown at the pointer less
//! its hot spot, or hidden. The engine blends it over the pipe, so the
//! compositor no longer redraws around the pointer.

use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::errno::{EINVAL, ENOMEM, ENXIO};
use nonos_gfx_wire::{Cursor, Errno};

use super::frames::Frames;
use super::intel::Intel;
use super::plug::Plug;
use crate::engine::access::{Clock, Mmio};
use crate::engine::cursor::set;
use crate::engine::ggtt_run::map_run;

/// The slot's 64 x 64 four-byte pixels: four GGTT entries.
const IMAGE: usize = 64 * 64 * 4;

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub fn show_cursor(&mut self, o: u16, c: &Cursor, slot: &[u8]) -> Result<(), Errno> {
        let Intel { m, heads, .. } = self;
        let h = heads.get_mut(o as usize).and_then(|h| h.as_mut()).ok_or(ENXIO)?;
        if h.plug != Plug::In {
            // A dark pipe shows no cursor; the repaint on LOST sends it again.
            return Ok(());
        }
        if c.flags & NEW_SHAPE != 0 {
            let img = slot.get(..IMAGE).ok_or(EINVAL)?;
            let iova = h.frames.cursor_image(img).ok_or(ENOMEM)?;
            h.frames.fence();
            if h.cursor.is_none() {
                let (first, top) = h.window;
                map_run(m, h.out.ggtt, first, iova, 4, top).map_err(|_| ENOMEM)?;
                h.cursor = Some(first * 4096);
            }
        }
        let base = h.cursor.filter(|_| c.flags & VISIBLE != 0);
        let (x, y) = (c.x - c.hot_x as i32, c.y - c.hot_y as i32);
        set(m, h.out.pipe, base, x, y).map_err(|_| EINVAL)
    }
}
