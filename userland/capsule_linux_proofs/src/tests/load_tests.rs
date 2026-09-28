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

/*
 * A family whose threads run all the time reads as Linux reads one task
 * that never sleeps: after a minute from idle, 1 - (1884/2048)^12 on the
 * one-minute average, and the longer averages behind it. One test only,
 * because the averages are the family's one state.
 */

use crate::calls::load::load::{averages, exited, text};

#[test]
fn a_minute_busy_then_a_minute_idle_reads_as_linux_does() {
    assert_eq!(averages(0, 0).map(text), ["0.00", "0.00", "0.00"]);
    /* 60 s at 100 Hz, every tick run. */
    let busy = averages(60_000, 6000);
    assert_eq!(busy.map(text), ["0.63", "0.18", "0.06"]);
    /* A process that exited took its ticks with it; they still count. */
    exited(6000);
    let idle = averages(120_000, 0);
    assert_eq!(idle.map(text), ["0.23", "0.15", "0.06"]);
    /* A read inside the same five seconds changes nothing. */
    assert_eq!(averages(124_999, 0), idle);
}
