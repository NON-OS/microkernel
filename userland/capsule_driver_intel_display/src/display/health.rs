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

//! STATE's look at the output (PLAN G 12): an underrun or a hung flip has
//! the plane pointed at the front surface again before the answer, which
//! says LOST so the compositor repaints; a stopped transcoder is beyond a
//! takeover's reach and stays LOST, not scanning out, until mode setting
//! can bring it back.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use super::frames::Frames;
use super::intel::{Intel, FLIP_US};
use crate::engine::access::{Clock, Mmio, Until};
use crate::engine::fault::{check, Fault};

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    /// STATE's flags now, with the generation bumped when they say
    /// something new: a change, or a loss the compositor must repaint.
    pub fn health(&mut self) -> u32 {
        let (p, t) = (self.out.pipe, self.out.transcoder);
        let front = self.scan.surfaces[self.scan.front];
        let flags = match check(&mut self.m, self.gen, p, t, front) {
            None => SCANNING_OUT,
            Some(Fault::Stopped) => LOST,
            Some(Fault::Underrun | Fault::Hung) => {
                let until = Until::new(&self.clock, FLIP_US);
                match self.scan.recover(&mut self.m, t, &until) {
                    Ok(()) => SCANNING_OUT | LOST,
                    Err(_) => LOST,
                }
            }
        };
        if flags != self.flags || flags & LOST != 0 {
            self.generation = self.generation.wrapping_add(1);
        }
        self.flags = flags;
        flags
    }
}
