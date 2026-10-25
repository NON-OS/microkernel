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

//! A holder H and a contender C of one transport lock, and a kill of H (L77e).
//! H takes the lock and sleeps through up to two rounds of its trip; killed,
//! it returns at once if on its CPU, and asleep it is never woken. C asks,
//! takes the lock over by the kernel's rule where it cannot have it, holds it
//! and lets go. `bad`: two inside at once, or a guard freeing another's lock.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum H {
    #[default]
    Out,
    Running,
    Asleep,
    Gone,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Knob {
    Built,
    NoTakeover,
    DeadOnly,
    OffCpuOnly,
    NoDeadReturn,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct World {
    pub locked: bool,
    pub row_h: bool,
    pub h: H,
    pub holds: bool,
    pub alive: bool,
    pub rounds: u8,
    pub c: u8,
    pub bad: bool,
}

impl World {
    pub fn start() -> Self {
        World { alive: true, rounds: 2, ..World::default() }
    }

    /// Path 0 the holder, 1 the contender, 2 the kill. False: it cannot move.
    pub fn step(&mut self, path: usize, k: Knob) -> bool {
        let moved = match path {
            0 => super::world_h::holder(self, k),
            1 => super::world_c::contender(self, k),
            _ => std::mem::replace(&mut self.alive, false),
        };
        // H is inside while it holds its guard and may still run.
        self.bad |= self.c == 1 && self.holds && (self.alive || self.h == H::Running);
        moved
    }

    pub fn drop_guard(&mut self) {
        self.bad |= self.c == 1;
        (self.locked, self.holds) = (false, false);
    }
}
