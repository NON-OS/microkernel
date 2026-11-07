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
use nonos_gfx_wire::errno::{ENXIO, ETIMEDOUT};
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, Scanout, State};

use super::copy::copy;
use super::direct::{leave, own_stride};
use super::describe;
use super::frames::Frames;
use super::intel::{Intel, FLIP_US};
use super::modes::modes;
use super::plug::Plug;
use crate::engine::access::{Clock, Mmio, Until};
use crate::engine::irq_regs::irq_regs;
use crate::engine::park::Park;

impl<M: Mmio, F: Frames, C: Clock, P: Park> Display for Intel<M, F, C, P> {
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
        let Intel { m, clock, gen, heads, park, blank, .. } = self;
        let h = heads.get_mut(output as usize).and_then(|h| h.as_mut()).ok_or(ENXIO)?;
        if h.plug != Plug::In {
            // Nothing is lit; the mode set that lights it copies it whole.
            // A client buffer it showed goes back on the primary's surface.
            if h.direct.shown.is_some() {
                own_stride(m, h);
                let front = h.scan.surfaces[h.scan.front];
                m.write(crate::engine::plane_regs::plane_surf(h.scan.pipe, 1), front);
                leave(m, h);
            }
            return Ok(());
        }
        let (size, stride) = ((h.scan.width, h.scan.height), h.scan.stride);
        // Back from a client's buffer, the primary's surfaces are drawn whole.
        let whole = [Rect { x: 0, y: 0, width: size.0, height: size.1 }];
        let rects = if h.direct.shown.is_some() { &whole[..] } else { rects };
        copy(&mut h.frames, h.scan.back(), size, stride, rects, shadow)?;
        h.frames.fence();
        let until = Until::new(clock, FLIP_US);
        let pipe = h.scan.pipe;
        own_stride(m, h);
        let irq = (irq_regs(gen, pipe), park as &mut dyn Park, &mut blank[pipe.index() as usize]);
        h.scan.present_parked(m, &until, irq).map_err(|_| ETIMEDOUT)?;
        // Plane 1 shows the primary: the client's buffer and the band go.
        leave(m, h);
        // Checked whole above, so the second copy cannot be refused.
        copy(&mut h.frames, h.scan.back(), size, stride, rects, shadow)
    }

    fn cursor(&mut self, output: u16, c: &Cursor, slot: &[u8]) -> Result<(), Errno> {
        self.show_cursor(output, c, slot)
    }

    fn scanout(&mut self, output: u16, s: &Scanout, shadow: &Shadow) -> Result<(), Errno> {
        self.scan_client(output, s, shadow)
    }
}
