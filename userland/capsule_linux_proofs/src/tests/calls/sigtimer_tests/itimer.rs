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

//! Interval timers and timerfd: each moves past every period that ended.

use crate::sigtimer::Itimer;

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
