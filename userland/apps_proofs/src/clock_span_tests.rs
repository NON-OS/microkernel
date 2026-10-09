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

//! Why the stopwatch and timer read the monotonic clock: they measure spans
//! from whatever base they are given, so a base that stands still (no wall
//! time yet) freezes them and a base that steps back (a time set on the Set
//! tab) eats a running span. Milliseconds since boot do neither.

use crate::clock_stopwatch::Stopwatch;
use crate::clock_timer::Timer;

#[test]
fn a_stopwatch_on_a_base_that_stands_still_never_moves() {
    // The wall-clock base with no time yet: every tick reads the same zero.
    let mut sw = Stopwatch::default();
    sw.toggle(0);
    assert_eq!(sw.elapsed(0), 0);
}

#[test]
fn a_wall_clock_set_back_under_a_running_stopwatch_eats_the_span() {
    let mut sw = Stopwatch::default();
    sw.toggle(1_800_000_000_000);
    // Thirty seconds in, the time is set back an hour on the wall clock.
    assert_eq!(sw.elapsed(1_800_000_030_000 - 3_600_000), 0);
}

#[test]
fn a_stopwatch_on_the_uptime_base_counts_the_span() {
    let mut sw = Stopwatch::default();
    sw.toggle(5_000);
    assert_eq!(sw.elapsed(35_000), 30_000);
    sw.toggle(35_000);
    assert_eq!(sw.elapsed(99_000), 30_000);
}

#[test]
fn a_timer_on_the_uptime_base_fires_at_its_deadline() {
    let mut timer = Timer::default();
    timer.set(60_000);
    timer.toggle(5_000);
    assert!(!timer.poll(64_999));
    assert_eq!(timer.remaining(64_999), 1);
    assert!(timer.poll(65_000));
    assert!(timer.fired);
}

#[test]
fn a_timer_on_a_base_that_stands_still_never_fires() {
    let mut timer = Timer::default();
    timer.set(60_000);
    timer.toggle(0);
    for _ in 0..1000 {
        assert!(!timer.poll(0));
    }
}
