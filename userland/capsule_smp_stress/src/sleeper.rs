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

//! Timed sleeps, which only the timer tick's sweep ends. One that returns
//! well past its deadline was woken late.

use crate::clock::now_ms;
use crate::stall;
use crate::stats::{self, Kind};
use nonos_libc::mk_idle_ms;

pub const THREADS: usize = 4;

pub fn run(arg: usize) {
    let mut turn = arg as u64;
    while !stats::stopping() {
        // 1 to 7 ms, so the deadlines fall on and between ticks.
        let asked = 1 + turn % 7;
        turn = turn.wrapping_add(1);
        let from = now_ms();
        if mk_idle_ms(asked) < 0 {
            stats::fault(Kind::Error);
            continue;
        }
        let slept = now_ms().saturating_sub(from);
        stats::round(Kind::Sleep, slept.saturating_sub(asked));
        if stall::is_late(asked, slept) {
            stats::fault(Kind::Late);
        }
    }
}
