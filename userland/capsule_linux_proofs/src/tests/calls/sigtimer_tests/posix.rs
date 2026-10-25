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

//! POSIX timers: overruns counted, a one-shot disarmed, and no wrap.

use crate::sigtimer::PosixTimer;

fn posix(due: u64, interval: u64) -> PosixTimer {
    PosixTimer {
        id: 0,
        clock: 1,
        signo: 34,
        tid: 0,
        value: 0,
        due: Some(due),
        interval,
        overrun: 0,
        last_overrun: 0,
        queued: false,
    }
}

#[test]
fn a_posix_timer_fired_late_counts_the_periods_it_missed() {
    let mut t = posix(1000, 100);
    assert!(t.rearm(1350));
    assert_eq!(t.overrun, 3);
    assert_eq!(t.due, Some(1400));
}

#[test]
fn a_posix_timer_fired_on_time_counts_nothing() {
    let mut t = posix(1000, 100);
    assert!(t.rearm(1000));
    assert_eq!(t.overrun, 0);
    assert_eq!(t.due, Some(1100));
}

#[test]
fn a_one_shot_posix_timer_disarms() {
    let mut t = posix(1000, 0);
    assert!(!t.rearm(1200));
    assert_eq!(t.due, None);
}

#[test]
fn a_posix_timer_with_an_interval_near_the_top_of_the_clock_never_wraps() {
    let mut t = posix(1000, u64::MAX / 2);
    assert!(t.rearm(5000));
    assert_eq!(t.due, Some(u64::MAX / 2 + 1000));
    assert!(t.rearm(u64::MAX / 2 + 1000));
    assert_eq!(t.due, Some(u64::MAX), "saturated, never back below now");
}
