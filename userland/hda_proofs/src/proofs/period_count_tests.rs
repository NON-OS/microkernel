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

//! A stream's periods are counted, not logged: an idle stream playing
//! silence is not an underrun, and a client that stops feeding is one
//! underrun however many silent periods follow before the stream stops.

use crate::periods::{Period, Periods};

const P: usize = 0x2000;

#[test]
fn an_idle_stream_has_no_underruns() {
    let mut p = Periods::new();
    for _ in 0..1000 {
        assert_eq!(p.period(0, P), Period::Idle);
    }
    assert_eq!((p.played, p.underruns), (0, 0));
    assert!(!p.any());
}

#[test]
fn one_dry_spell_is_one_underrun() {
    let mut p = Periods::new();
    for _ in 0..10 {
        assert_eq!(p.period(P, P), Period::Played);
    }
    assert_eq!(p.period(P / 2, P), Period::Underrun);
    for _ in 0..50 {
        assert_eq!(p.period(0, P), Period::Idle);
    }
    assert_eq!((p.played, p.underruns), (10, 1));
    assert!(p.any());
}

#[test]
fn each_return_to_flowing_can_run_dry_again() {
    let mut p = Periods::new();
    let feed = [P, 0, 0, P, P, 100, P, 0];
    let want = [
        Period::Played,
        Period::Underrun,
        Period::Idle,
        Period::Played,
        Period::Played,
        Period::Underrun,
        Period::Played,
        Period::Underrun,
    ];
    for (f, w) in feed.iter().zip(want) {
        assert_eq!(p.period(*f, P), w);
    }
    assert_eq!((p.played, p.underruns), (4, 3));
}
