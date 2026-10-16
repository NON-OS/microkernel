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

//! A gathered flight ends when the far end finishes, past its size, or at a break.

use super::gather_rig::{never, BOUNDS};
use crate::wallet::net::step::gather::{Gather, Gathered};

#[test]
fn a_finished_far_end_ends_the_flight_at_once() {
    let mut g = Gather::new(BOUNDS, 0);
    assert_eq!(g.take(b"last", true, 5, never), Gathered::Whole(b"last".to_vec()));
    let mut empty = Gather::new(BOUNDS, 0);
    assert_eq!(empty.take(b"", true, 5, never), Gathered::Nothing);
}

#[test]
fn more_than_a_flight_may_hold_is_refused() {
    let mut g = Gather::new(BOUNDS, 0);
    assert_eq!(g.take(&[7; 16], false, 1, never), Gathered::More);
    assert_eq!(g.take(&[7], false, 2, never), Gathered::Nothing);
}

#[test]
fn a_broken_stream_keeps_what_came_before_the_break() {
    let mut g = Gather::new(BOUNDS, 0);
    assert_eq!(g.take(b"part", false, 1, never), Gathered::More);
    assert_eq!(g.broke(), Gathered::Whole(b"part".to_vec()));
    assert_eq!(Gather::new(BOUNDS, 0).broke(), Gathered::Nothing);
}
