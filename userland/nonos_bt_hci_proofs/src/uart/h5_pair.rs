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

//! Two Three-wire sides joined by a line that loses packets on purpose:
//! each frame either way is lost with chance `lose` in 100, drawn from a
//! fixed pseudo-random sequence so every run is the same. A lost frame
//! never arrives at all.

use nonos_bt_hci::uart::h5::H5;

pub const RETX_MS: u64 = 10;

pub struct Pair {
    pub a: H5,
    pub b: H5,
    pub now: u64,
    pub lose: u64,
    seed: u64,
}

impl Pair {
    pub fn new(lose: u64) -> Self {
        Self { a: H5::default(), b: H5::default(), now: 0, lose, seed: 0x9e37_79b9_7f4a_7c15 }
    }

    /// One millisecond: each side sends what is due, the line delivers it.
    pub fn tick(&mut self) {
        self.now += 1;
        while let Some(o) = self.a.poll_out(self.now, RETX_MS) {
            if self.keep() {
                self.b.feed(&o, self.now);
            }
        }
        while let Some(o) = self.b.poll_out(self.now, RETX_MS) {
            if self.keep() {
                self.a.feed(&o, self.now);
            }
        }
    }

    fn keep(&mut self) -> bool {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed % 100 >= self.lose
    }

    pub fn until(&mut self, ticks: u32, done: impl Fn(&Pair) -> bool) -> bool {
        (0..ticks).any(|_| {
            self.tick();
            done(self)
        })
    }
}
