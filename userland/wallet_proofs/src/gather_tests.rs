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

//! A flight gathered a slice at a time keeps the routed reader's bounds:
//! whole once `done` says so or the far end finished, ended by the first
//! wait with nothing, by the quiet window after a piece, and by the total,
//! and refused past its size. The clock is the test's, so every bound is
//! met exactly.

use super::gather_rig::{never, BOUNDS};
use crate::wallet::net::routed_read::Bounds;
use crate::wallet::net::step::gather::{Gather, Gathered};

#[test]
fn a_flight_is_whole_once_done_says_so() {
    let mut g = Gather::new(BOUNDS, 0);
    assert_eq!(g.take(b"ab", false, 10, |b| b.len() >= 4), Gathered::More);
    assert_eq!(g.take(b"cd", false, 20, |b| b.len() >= 4), Gathered::Whole(b"abcd".to_vec()));
}

#[test]
fn nothing_inside_the_first_wait_is_waited_for_then_nothing() {
    let mut g = Gather::new(BOUNDS, 100);
    assert_eq!(g.take(b"", false, 1_099, never), Gathered::More);
    assert_eq!(g.take(b"", false, 1_100, never), Gathered::Nothing);
}

#[test]
fn the_quiet_window_after_a_piece_ends_the_flight() {
    let mut g = Gather::new(BOUNDS, 0);
    assert_eq!(g.take(b"abc", false, 500, never), Gathered::More);
    assert_eq!(g.take(b"", false, 699, never), Gathered::More);
    assert_eq!(g.take(b"", false, 700, never), Gathered::Whole(b"abc".to_vec()));
}

#[test]
fn a_far_end_that_keeps_trickling_is_cut_at_the_total() {
    let mut g = Gather::new(Bounds { max: 1 << 20, ..BOUNDS }, 0);
    let mut now = 0;
    let mut got = Gathered::More;
    while got == Gathered::More {
        now += 150;
        got = g.take(b"x", false, now, never);
    }
    assert_eq!(now, 5_100, "the first slice at or past the total ends it");
    let Gathered::Whole(bytes) = got else { panic!("what came is kept") };
    assert_eq!(bytes.len(), 34);
}
