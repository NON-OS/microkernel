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

//! A send: the run moved out of PENDING and sent with no lock, or, under
//! the old hold, sent with PENDING still held. The sender sleeps until the
//! reply; a send carrying a sector some other caller held is FOREIGN.

use super::world::{Knob, Ph, World, FOREIGN};

impl World {
    pub(super) fn send(&mut self, a: usize, k: Knob, next: bool) {
        let locked = k == Knob::SendUnderLock;
        let sending = if locked { self.run.clone() } else { core::mem::take(&mut self.run) };
        self.lock = locked.then_some(a);
        self.ph[a] = Ph::Asleep { sending, locked, next };
    }

    pub(super) fn woken(&self, a: usize) -> World {
        let mut w = self.clone();
        let Ph::Asleep { sending, locked, next } = &self.ph[a] else { unreachable!() };
        w.bad |= FOREIGN * u8::from(sending.iter().any(|s| s.0 != a));
        if *locked {
            (w.run, w.lock) = (Vec::new(), None);
        }
        w.ph[a] = Ph::Run;
        w.pc[a] += usize::from(*next);
        w
    }
}
