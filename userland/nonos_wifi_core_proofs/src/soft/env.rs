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

//! The station's clock and randomness in the simulation: the clock moves a
//! millisecond each time it is read, so every bounded wait in the engine
//! ends; the randomness is a fixed sequence, so every run is the same.

use std::cell::Cell;
use std::rc::Rc;

use nonos_wifi_core::softmac::Env;

pub struct SimEnv {
    pub clock: Rc<Cell<u64>>,
    pub draws: u8,
}

impl Env for SimEnv {
    fn now_ms(&self) -> u64 {
        self.clock.set(self.clock.get() + 1);
        self.clock.get()
    }

    fn random(&mut self, out: &mut [u8]) -> bool {
        for b in out.iter_mut() {
            self.draws = self.draws.wrapping_add(1);
            *b = self.draws.wrapping_mul(151).wrapping_add(7);
        }
        true
    }
}
