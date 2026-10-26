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

//! A caller's own step, and the bad states a move reached.

use super::world::{pid, Knob, World, P};

impl World {
    /// Caller `i` takes its next step, if it still runs.
    pub(super) fn run(&mut self, i: usize, k: Knob) -> bool {
        let runs = (self.alive[i] && !(i == 2 && self.stopped)) || self.p[i] == P::In;
        if !runs {
            return false;
        }
        match self.p[i] {
            P::Out | P::Waiting if self.alive[i] => self.try_in(i, k),
            P::In if !self.alive[i] => self.leave(i, k),
            P::In if self.rounds[i] > 0 => (self.rounds[i], self.p[i]) = (0, P::Asleep),
            P::In => self.leave(i, k),
            P::Asleep if self.alive[i] => self.p[i] = P::In,
            _ => return false,
        }
        true
    }

    /// Caller `i` killed: one inside on one CPU dies only where it rests.
    pub(super) fn kill(&mut self, i: usize) -> bool {
        if !self.alive[i] || self.p[i] == P::Done || (self.up && self.p[i] == P::In) {
            return false;
        }
        self.alive[i] = false;
        if matches!(self.p[i], P::Waiting | P::Out) {
            self.p[i] = P::Parked;
        }
        true
    }

    /// H's pid handed to a new process: its rows forgotten.
    pub(super) fn reuse(&mut self, k: Knob) {
        self.reused = true;
        if k != Knob::NoForget {
            let keep = self.rows.waiting.clone();
            if self.rows.sweep(|q| q == pid(0)) {
                self.held = None;
            }
            if k == Knob::ForgetKeepsWaiting {
                self.rows.waiting = keep;
            }
        }
    }
}
