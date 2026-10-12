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

use super::{mk_idle_ms, Deadline};

/// How long a wait spins before it sleeps between looks. A command on a
/// solid-state disk and a register handshake end inside it.
const SPIN_MS: u64 = 1;

/// The pace of one wait's looks: a spin for its first SPIN_MS, then a 1 ms
/// sleep between looks, so a slow device does not hold a core for the whole
/// of a wait that may last 30 s.
pub struct Pace {
    spin: Deadline,
}

impl Pace {
    pub fn start() -> Self {
        Self { spin: Deadline::after_ms(SPIN_MS) }
    }

    /// Between two looks.
    pub fn rest(&self) {
        if self.spin.expired() {
            let _ = mk_idle_ms(1);
        } else {
            core::hint::spin_loop();
        }
    }
}

/// Wait until `cond` holds or `ms` milliseconds pass. True when it held; the
/// condition is read once more after the deadline, so a register that
/// settled while the clock was read is not taken for a timeout.
pub fn wait_until(ms: u64, mut cond: impl FnMut() -> bool) -> bool {
    let deadline = Deadline::after_ms(ms);
    let pace = Pace::start();
    loop {
        if cond() {
            return true;
        }
        if deadline.expired() {
            return cond();
        }
        pace.rest();
    }
}

/// Wait `ms` milliseconds.
pub fn pause_ms(ms: u64) {
    let deadline = Deadline::after_ms(ms);
    let pace = Pace::start();
    while !deadline.expired() {
        pace.rest();
    }
}
