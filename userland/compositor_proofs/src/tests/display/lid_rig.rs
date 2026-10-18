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

//! Frames as the tick runs them under the lid: nothing when dark, else each
//! drained rectangle composed (black while closed, the desktop and the band
//! open) and presented, a refused batch kept damaged.

pub(super) use super::lid_paint::{black, whole, Shows, H, W};
use super::lid_paint::{paint, present};
use crate::damage::{DamageAccumulator, Rect};
use crate::frame_pacer::drain_batch::drain_batched;
use crate::state::lid_rule::Lid;

pub(super) struct Rig {
    pub(super) canvas: Vec<u32>,
    pub(super) screen: Vec<u32>,
    pub(super) acc: DamageAccumulator,
    pub(super) lid: Lid,
    pub(super) presents: u32,
    pub(super) refuse: u32,
}

impl Rig {
    pub(super) fn new(s: Shows) -> Rig {
        let (canvas, screen) = (whole(Lid::Open, s), whole(Lid::Open, s));
        Rig {
            canvas,
            screen,
            acc: DamageAccumulator::new(),
            lid: Lid::Open,
            presents: 0,
            refuse: 0,
        }
    }

    /// The shell's LID, as the handler takes it.
    pub(super) fn shell(&mut self, closed: bool) {
        let (lid, whole) = self.lid.moved(closed);
        self.lid = lid;
        if whole {
            self.acc.mark_full(W, H);
        }
    }

    pub(super) fn frame(&mut self, s: Shows) {
        if !self.lid.presents() {
            return;
        }
        let lid = self.lid;
        let prepare = |r: &mut Rig, rect: Rect| {
            paint(&mut r.canvas, rect, lid, s);
            Some(rect)
        };
        let done = drain_batched(self, |r| &mut r.acc, prepare, present);
        self.lid = self.lid.after_frame(done.is_err());
    }
}
