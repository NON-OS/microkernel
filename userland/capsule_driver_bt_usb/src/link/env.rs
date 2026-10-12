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

//! What the link needs from the capsule around it: the clock, the input
//! ring, the debug line, the user, and the random source pairing draws
//! from. The proofs stand in for all five, so every path the link takes
//! runs on the host.

use nonos_bt_hogp::report::Input;
use nonos_bt_smp::pairing::Random;

pub trait Env: Random {
    /// Milliseconds since some fixed point; never goes back.
    fn now_ms(&mut self) -> u64;
    /// One event for the input ring.
    fn input(&mut self, input: Input);
    /// Once a turn of the link's loop, for whatever repeats on the clock.
    fn tick(&mut self, now_ms: u64);
    /// One line for whoever reads the capsule's log.
    fn say(&mut self, words: &[&[u8]]);
    /// Six digits for the user to type on the keyboard being paired.
    fn show_passkey(&mut self, passkey: u32);
}
