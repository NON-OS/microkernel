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

//! A caller coming in or kept out, and a caller leaving: sends what it held
//! before reading or flushing, then lets go and wakes who waits.

use super::world::{pid, Knob, World, NOW, P, WHOLE};

impl World {
    pub(super) fn try_in(&mut self, i: usize, k: Knob) {
        let (me, whole) = (pid(i), WHOLE[i]);
        self.woken[i] = false;
        if !self.may(i, k) {
            let snap = self.clone();
            let next = self.rows.next;
            if self.rows.sweep(|q| snap.gone(q, k)) {
                self.bad |= 0x01 * (0..4).any(|j| j != i && self.live_inside(j)) as u16;
                if k != Knob::NoPutRight {
                    self.held = None;
                }
            }
            if matches!(k, Knob::NoNextSweep | Knob::NoNextSweepNeverStale) {
                self.rows.next = next;
            }
        }
        if self.may(i, k) {
            self.rows.take(me, whole);
            self.p[i] = P::In;
        } else {
            self.rows.wait(me, whole, NOW);
            self.p[i] = P::Waiting;
        }
    }

    pub(super) fn leave(&mut self, i: usize, k: Knob) {
        let me = pid(i);
        self.bad |= 0x02 * self.held.is_some_and(|q| q != me) as u16;
        if self.held == Some(me) && !self.failed[i] {
            self.held = None;
        }
        if WHOLE[i] && k != Knob::NoLetGo {
            self.held = None;
        }
        let (had, mut woken) = self.rows.give(me, WHOLE[i]);
        if k == Knob::NoWakeList {
            woken.clear();
        }
        self.bad |= 0x08 * !had as u16;
        for w in woken {
            self.bad |= 0x20 * (w == pid(0) && self.reused) as u16;
            if let Some(j) = (0..4).find(|&j| pid(j) == w) {
                self.woken[j] = true;
            }
        }
        self.p[i] = P::Done;
    }
}
