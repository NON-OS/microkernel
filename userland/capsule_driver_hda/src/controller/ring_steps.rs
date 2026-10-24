// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The recovery's two steps (ring_recover.rs): a reset in place, or the
//! rings given up for the Immediate Command interface.

use super::ring_recover::RESETS_BEFORE_IMMEDIATE;
use super::ring_reset::{restart, stop};
use super::verb::Link;
use crate::setup::Line;

impl Link {
    /// Reset the rings, or give them up when resets have stopped helping or
    /// an engine will not stop.
    pub(super) fn recover(&mut self) {
        self.health.unfetched = 0;
        if self.health.resets_unanswered >= RESETS_BEFORE_IMMEDIATE || !restart(self.regs) {
            self.give_up();
            return;
        }
        (self.wp, self.rp) = (0, 0);
        self.health.resets = self.health.resets.saturating_add(1);
        self.health.resets_unanswered = self.health.resets_unanswered.saturating_add(1);
        Line::new("[HDA] verbs went unanswered; rings reset, ")
            .dec(self.health.resets)
            .s(" so far")
            .emit();
    }

    fn give_up(&mut self) {
        // The Immediate interface may not run beside the CORB (3.4.3). When
        // an engine will not stop it is taken all the same: it is the only
        // way left to the codec, a CORB that stopped fetching (why this runs)
        // takes no part in it, and each exchange is bounded (immediate.rs),
        // so the worst it brings is a verb that times out.
        let stopped = stop(self.regs);
        self.health.immediate = true;
        let how = if stopped { "rings stopped" } else { "rings would not stop" };
        Line::new("[HDA] verb rings given up (").s(how).s("); one verb at a time from now").emit();
    }
}
