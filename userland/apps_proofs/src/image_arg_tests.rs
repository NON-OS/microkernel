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

//! The image viewer asked the shell for a path every 150 ms for as long as it
//! was open. It asks on every tick only while it has nothing to show, then
//! about once a second, and a file opened while it is up still shows within
//! a second.

use crate::viewer::arg_cadence::{next_ask, EVERY_MS, STARTUP_MS, TICK_MS};

const START: i64 = 40_000;

/// Run the viewer's ticks from START until `end`, asking whenever an ask is
/// due, as `poll_open` does. `showing_from` is when the gallery or an image
/// first shows. Returns the tick times at which it asked.
fn asks(end: i64, showing_from: i64) -> Vec<i64> {
    let mut due = 0;
    let mut out = Vec::new();
    let mut now = START;
    while now < end {
        if now >= due {
            due = next_ask(now, START, now >= showing_from);
            out.push(now);
        }
        now += TICK_MS;
    }
    out
}

/// When a path left at `left` is taken: the first ask at or after it.
fn taken(asks: &[i64], left: i64) -> i64 {
    *asks.iter().find(|t| **t >= left).expect("asked again after the path was left")
}

#[test]
fn an_empty_window_asks_on_every_tick_while_it_starts() {
    let quick = asks(START + STARTUP_MS, i64::MAX);
    assert_eq!(quick.len() as i64, STARTUP_MS / TICK_MS);
    assert!(quick.windows(2).all(|w| w[1] - w[0] == TICK_MS));
}

#[test]
fn a_file_handed_at_launch_is_taken_on_the_first_tick() {
    assert_eq!(taken(&asks(START + 1_000, i64::MAX), START), START);
}

#[test]
fn once_something_shows_it_asks_about_once_a_second() {
    // The gallery shows after the first tick, as it does when scanned.
    let a = asks(START + 60_000, START + TICK_MS);
    // The old poll asked 400 times a minute.
    assert!((60..=70).contains(&a.len()), "{} asks in a minute", a.len());
    for w in a.windows(2).skip(2) {
        assert!(w[1] - w[0] >= EVERY_MS && w[1] - w[0] <= 1_000, "{} ms apart", w[1] - w[0]);
    }
}

#[test]
fn a_window_with_nothing_to_show_slows_down_after_starting_too() {
    let a = asks(START + STARTUP_MS + 60_000, i64::MAX);
    let late = a.iter().filter(|t| **t >= START + STARTUP_MS).count();
    assert!((60..=70).contains(&late), "{late} asks in the minute after starting");
}

#[test]
fn a_file_handed_to_an_open_viewer_shows_within_a_second() {
    let a = asks(START + 30_000, START + TICK_MS);
    for offset in (0..3_000).step_by(7) {
        let left = START + 10_000 + offset;
        let wait = taken(&a, left) - left;
        assert!(wait <= 1_000, "a path left at +{offset} ms waited {wait} ms");
    }
}
