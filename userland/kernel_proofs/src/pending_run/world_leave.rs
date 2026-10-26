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

//! The steps that give the volume back and read: a writer's give lets go
//! of what it held, and a read of a sector still held is PAST.

use super::world::{Knob, Vol, World, PAST};

impl World {
    pub(super) fn leave_writer(&mut self, a: usize, k: Knob) {
        self.let_go(k);
        self.vol = Vol::Free;
        self.pc[a] += 1;
    }

    pub(super) fn read(&mut self, a: usize, lba: u64) {
        self.bad |= PAST * u8::from(self.run.iter().any(|s| s.1 == lba));
        self.pc[a] += 1;
    }

    pub(super) fn leave_reader(&mut self, a: usize) {
        self.vol = match self.vol {
            Vol::Readers(n) if n > 1 => Vol::Readers(n - 1),
            _ => Vol::Free,
        };
        self.pc[a] += 1;
    }
}
