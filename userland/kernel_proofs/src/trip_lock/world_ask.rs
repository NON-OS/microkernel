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

//! What the model asks of a world: on a CPU, inside, gone by the kernel's
//! takeover rule (or a knob's broken one), and may come in.

use super::transport_rule::takes_over;
use super::world::{pid, Knob, World, NOW, P, WHOLE};

impl World {
    pub(super) fn on_cpu(&self, i: usize) -> bool {
        !self.up && self.p[i] == P::In
    }
    pub(super) fn live_inside(&self, i: usize) -> bool {
        self.p[i] == P::In || (self.p[i] == P::Asleep && self.alive[i])
    }
    /// The kernel's own takeover rule, or the knob's broken one.
    pub(super) fn gone(&self, q: u32, k: Knob) -> bool {
        let Some(i) = (0..4).find(|&i| pid(i) == q) else { return false };
        let ended = !self.alive[i] && (i != 0 || !self.reused);
        let on_cpu = self.on_cpu(i);
        match k {
            Knob::NoSweep => false,
            Knob::EndedOnly => ended,
            Knob::OffCpuOnly => !on_cpu,
            _ => takes_over(q, ended, on_cpu),
        }
    }
    pub(super) fn may(&self, i: usize, k: Knob) -> bool {
        let (me, whole) = (pid(i), WHOLE[i]);
        if k == Knob::NoNestedRead && !whole && self.rows.writer.is_none() {
            return self.rows.next_at(NOW).is_none();
        }
        self.rows.may_take(me, whole, NOW)
    }
}
