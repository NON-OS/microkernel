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

//! The arithmetic of the timers that end in a signal: a periodic timer moves
//! past every period that ended while it was not looked at, a one-shot one
//! stops, and a POSIX timer counts the periods it missed as overruns, as
//! Linux's timer_getoverrun reports them.

use crate::sigtimer::{Itimer, PosixTimer};

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
fn a_periodic_itimer_moves_past_every_period_that_ended() {
    let mut t = Itimer { due: 1000, interval: 200 };
    assert!(t.rearm(1450));
    assert_eq!(t.due, 1600);
}

#[test]
fn a_one_shot_itimer_does_not_fire_again() {
    let mut t = Itimer { due: 1000, interval: 0 };
    assert!(!t.rearm(1000));
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

#[test]
fn a_timerfd_with_an_interval_near_the_top_of_the_clock_fires_once_and_waits() {
    use crate::linux::guest::timer::Timer;
    let mut t = Timer { due: 1000, every: u64::MAX - 10, clock: 1 };
    assert_eq!(t.take(1500), 1);
    assert!(t.due > 1500, "the next time is after now, not wrapped before it");
    assert_eq!(t.take(1600), 0, "and it does not fire again at the next look");
    let mut t = Timer { due: 1000, every: 100, clock: 1 };
    assert_eq!(t.take(1350), 4);
    assert_eq!(t.due, 1400);
}
