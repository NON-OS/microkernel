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

//! A virtual clock per test thread. Each read moves it on by the cost of one
//! poll over the bus and each sleep by what was asked, so a ten-second
//! timeout runs in no time on the host and a test can count how a wait
//! spent its time: polls (reads) against parked sleeps.

use std::cell::Cell;

/// What one read of the clock stands for: a register poll on the bus.
const READ_US: u64 = 2;

thread_local! {
    static NOW_US: Cell<u64> = const { Cell::new(0) };
    static READS: Cell<u64> = const { Cell::new(0) };
    static SLEEPS: Cell<u64> = const { Cell::new(0) };
}

pub fn mk_uptime_ms() -> i64 {
    READS.with(|r| r.set(r.get() + 1));
    NOW_US.with(|n| {
        n.set(n.get() + READ_US);
        (n.get() / 1000) as i64
    })
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    SLEEPS.with(|s| s.set(s.get() + 1));
    advance_ms(ms);
    0
}

pub fn advance_ms(ms: u64) {
    NOW_US.with(|n| n.set(n.get() + ms * 1000));
}

pub fn now_ms() -> u64 {
    NOW_US.with(|n| n.get() / 1000)
}

/// Clock reads on this thread so far.
pub fn uptime_reads() -> u64 {
    READS.with(|r| r.get())
}

/// Parked sleeps on this thread so far.
pub fn sleeps() -> u64 {
    SLEEPS.with(|s| s.get())
}
