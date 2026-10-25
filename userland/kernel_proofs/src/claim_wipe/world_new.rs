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

//! A seed's world: its function (world_function), then A, A2 and B with
//! their names, each the least `driver.` name `name_of` finds among what it
//! serves (A2's in the other order); when each first asks, how many times,
//! and whether a kill lands; and the two CPUs' first ticks.

use super::holder_rule::name_of;
use super::method_rule::Method;
use super::phase_rule::MS;
use super::world::World;
use super::world_knob::Knob;
use super::world_thread::{Path, Proc, Thread};

impl World {
    pub(super) fn new(seed: u64, m: Option<Method>, knob: Knob) -> World {
        let rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let mut w = World { rng, knob, epochs: 1, ..World::default() };
        w.dev = w.function(m);
        w.people();
        w
    }

    fn people(&mut self) {
        let one: &[&str] = &["net.e1000"];
        let a = if self.chance(8) { one } else { &["net.e1000", "driver.e1000_0", "driver.zz"] };
        // B with no `driver.` name serves the one A served outside that root,
        // as another capsule may once A is gone: only the root names a service.
        let b = if self.chance(6) { one } else { &["driver.rtl8139_0"] };
        let (ours, again) = (name_of(a.iter().copied()), name_of(a.iter().rev().copied()));
        for (p, name) in [ours, again, name_of(b.iter().copied())].into_iter().enumerate() {
            let (svc, attempts) = ((p == 2) as u8, 1 + self.draw(3) as u8);
            self.procs[p] = Proc { svc, name, attempts, ..Proc::default() };
            let at = if p == 0 { 0 } else { MS * self.draw(4001) };
            self.threads[3 * p] = Thread { path: Path::Idle, at, ..Thread::default() };
            if self.chance(4) {
                let at = MS * self.draw(6001);
                self.threads[3 * p + 2] = Thread { path: Path::Exit, at, ..Thread::default() };
            }
        }
        (self.threads[9].at, self.threads[10].at) = (MS * self.draw(41), MS * self.draw(41));
    }
}
