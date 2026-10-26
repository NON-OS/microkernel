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

//! One move of the world: a caller runs, a kill, a pid reused, a dead holder
//! parked, the clock staling a mark, C stopped, an error path, a nested read.

use super::trip_rows::FRESH_MS;
use super::world::{pid, Knob, World, NOW, P};

impl World {
    pub(super) fn step(&mut self, a: usize, k: Knob) -> bool {
        match a {
            0..=3 => return self.run(a, k) && self.judge(),
            4..=6 => return self.kill(a - 4) && self.judge(),
            7 if !self.alive[0] && !self.reused && self.p[0] != P::In => self.reuse(k),
            8..=11 if self.p[a - 8] == P::In && !self.alive[a - 8] => self.p[a - 8] = P::Asleep,
            12 => {
                let stale = !matches!(k, Knob::NeverStale | Knob::NoNextSweepNeverStale);
                let Some(n) = self.rows.next.filter(|n| n.1 == NOW && stale) else { return false };
                self.rows.next = Some((n.0, NOW - FRESH_MS - 1));
            }
            13 if self.p[2] == P::Waiting && self.alive[2] && !self.stop_used => {
                (self.stopped, self.stop_used) = (true, true);
            }
            15 | 16 => {
                let i = [0, 2][a - 15];
                if self.p[i] != P::In || !self.alive[i] || self.failed[i] {
                    return false;
                }
                self.failed[i] = true;
                self.held = Some(pid(i));
            }
            17 | 18 => {
                let i = [1, 3][a - 17];
                if self.p[i] != P::In || self.nested[i] == 0 {
                    return false;
                }
                self.nested[i] -= 1;
                self.bad |= 0x10 * !self.may(i, k) as u16;
            }
            _ => return false,
        }
        self.judge()
    }
}
