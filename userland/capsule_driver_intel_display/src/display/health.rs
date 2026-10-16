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

//! STATE's look at an output (PLAN G 12): an underrun or a hung flip has
//! the plane pointed at the front surface again, a lost DisplayPort link is
//! trained again (`link_keep`), each answered LOST so the compositor
//! repaints; a stopped transcoder stays LOST, not scanning out. A pulled
//! output is dark (no flags) and one plugged back LOST (`plug`).

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::errno::ENXIO;
use nonos_gfx_wire::Errno;

use super::frames::Frames;
use super::intel::{Intel, FLIP_US};
use super::link_keep::keep_link;
use super::plug::Plug;
use crate::engine::access::{Clock, Mmio, Until};
use crate::engine::fault::{check, Fault};

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    /// Output `o`'s STATE flags now, its generation bumped when they say
    /// something new: a change, or a loss the compositor must repaint.
    pub fn health(&mut self, o: u16) -> Result<u32, Errno> {
        let Intel { m, clock, gen, heads, .. } = self;
        let h = heads.get_mut(o as usize).and_then(|h| h.as_mut()).ok_or(ENXIO)?;
        let (p, t) = (h.out.pipe, h.out.transcoder);
        let flags = match h.plug {
            // Pulled, nothing shows; back, it is lost until a mode set.
            Plug::Out => 0,
            Plug::Back => LOST,
            Plug::In => match check(m, gen, p, t, h.scan.surfaces[h.scan.front]) {
                None => match keep_link(m, clock, h) {
                    None => SCANNING_OUT,
                    Some(true) => SCANNING_OUT | LOST,
                    Some(false) => LOST,
                },
                Some(Fault::Stopped) => LOST,
                Some(Fault::Underrun | Fault::Hung) => {
                    let until = Until::new(clock, FLIP_US);
                    match h.scan.recover(m, t, &until) {
                        Ok(()) => SCANNING_OUT | LOST,
                        Err(_) => LOST,
                    }
                }
            },
        };
        if flags != h.flags || flags & LOST != 0 {
            h.generation = h.generation.wrapping_add(1);
        }
        h.flags = flags;
        Ok(flags)
    }
}
