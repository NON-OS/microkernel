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

//! A clock that cannot be read ends every wait instead of holding it: the
//! platform clock answers u64::MAX for a refused read, and every deadline
//! and pause in the engine is measured against it.

use std::cell::Cell;

use crate::emmc::env::{pause, Clock, Deadline};

/// Reads 0, 1, 2, ... and, from read `dies_at` on, fails.
struct Dying {
    reads: Cell<u64>,
    dies_at: u64,
}

impl Clock for Dying {
    fn now_ms(&self) -> u64 {
        let n = self.reads.get();
        self.reads.set(n + 1);
        if n >= self.dies_at {
            u64::MAX
        } else {
            n
        }
    }
}

fn dying(dies_at: u64) -> Dying {
    Dying { reads: Cell::new(0), dies_at }
}

#[test]
fn a_dead_clock_has_passed_every_deadline() {
    let dead = dying(0);
    assert!(Deadline::after(&dead, 1_000).passed(&dead));
    assert!(Deadline::after(&dead, 0).passed(&dead));
    let before = dead.reads.get();
    pause(&dead, u64::MAX);
    assert_eq!(dead.reads.get() - before, 2, "one read to set, one to pass");
}

#[test]
fn a_clock_that_dies_mid_wait_ends_it_at_the_next_read() {
    let c = dying(5);
    pause(&c, 1_000_000);
    assert_eq!(c.reads.get(), 6);
    let c = dying(3);
    let d = Deadline::after(&c, 1_000_000);
    assert!(!d.passed(&c) && !d.passed(&c));
    assert!(d.passed(&c));
}

#[test]
fn a_working_clock_pauses_one_tick_past_the_time_asked() {
    let c = dying(u64::MAX);
    pause(&c, 10);
    // The start, then reads 1 to 11: 11 is the first past 0 + 10.
    assert_eq!(c.reads.get(), 12);
}
