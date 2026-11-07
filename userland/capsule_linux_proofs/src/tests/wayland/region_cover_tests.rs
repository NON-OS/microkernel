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

//! Whether a wl_region covers a whole surface, so the Wayland bridge may
//! call a pinned window opaque (region_cover.rs): never where it may not.

use crate::wayland_region_cover::{Step, Steps, MAX_STEPS};

/// An add (true) or a subtract, with its rectangle.
type Said = (bool, (i32, i32, i32, i32));

fn steps(list: &[Said]) -> Steps {
    let mut s = Steps::default();
    for &(add, rect) in list {
        s.push(Step { add, rect });
    }
    s
}

const W: u32 = 640;
const H: u32 = 480;

#[test]
fn an_empty_region_covers_nothing() {
    assert!(!Steps::default().covers(W, H));
}

#[test]
fn one_rectangle_holding_the_surface_covers_it() {
    assert!(steps(&[(true, (0, 0, 640, 480))]).covers(W, H));
    assert!(steps(&[(true, (-5, -5, 700, 500))]).covers(W, H));
    assert!(!steps(&[(true, (i32::MIN, i32::MIN, i32::MAX, i32::MAX))]).covers(1, 1));
    assert!(steps(&[(true, (0, 0, i32::MAX, i32::MAX))]).covers(W, H));
}

#[test]
fn a_rectangle_short_of_the_surface_does_not() {
    assert!(!steps(&[(true, (0, 0, 639, 480))]).covers(W, H));
    assert!(!steps(&[(true, (1, 0, 640, 480))]).covers(W, H));
    assert!(!steps(&[(true, (0, 0, 640, -480))]).covers(W, H));
    let halves = [(true, (0, 0, 320, 480)), (true, (320, 0, 320, 480))];
    assert!(!steps(&halves).covers(W, H), "a union is not counted");
}

#[test]
fn a_subtract_meeting_the_surface_takes_the_cover_away_until_an_add_holds_it_again() {
    let whole = (true, (0, 0, 640, 480));
    assert!(!steps(&[whole, (false, (639, 479, 1, 1))]).covers(W, H));
    assert!(!steps(&[whole, (false, (-10, -10, 11, 11))]).covers(W, H));
    assert!(steps(&[whole, (false, (640, 0, 10, 480))]).covers(W, H), "outside");
    assert!(steps(&[whole, (false, (0, 0, 0, 480))]).covers(W, H), "empty");
    assert!(steps(&[whole, (false, (0, 0, 5, 5)), whole]).covers(W, H));
    assert!(!steps(&[(false, (0, 0, 5, 5)), (true, (0, 0, 10, 10))]).covers(W, H));
}

#[test]
fn a_region_of_more_steps_than_kept_covers_nothing() {
    let mut s = steps(&[(true, (0, 0, 640, 480))]);
    for _ in 1..MAX_STEPS {
        s.push(Step { add: true, rect: (0, 0, 1, 1) });
    }
    assert!(s.covers(W, H), "all kept");
    s.push(Step { add: true, rect: (0, 0, 640, 480) });
    assert!(!s.covers(W, H), "one past what is kept");
}

#[test]
fn a_surface_of_no_size_is_never_covered() {
    let s = steps(&[(true, (0, 0, 640, 480))]);
    assert!(!s.covers(0, H) && !s.covers(W, 0));
}
