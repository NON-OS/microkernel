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

use crate::wallet::net::routed_read::{read_routed, Bounds, Source};
use crate::wallet::net::step::gather::{Gather, Gathered};

const BOUNDS: Bounds = Bounds { first_ms: 1_000, quiet_ms: 200, total_ms: 5_000, max: 16 };

/* The window's tick between slices. */
const TICK: i64 = 30;

fn never(_: &[u8]) -> bool {
    false
}

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

/// Pieces that land at set times, read either by the blocking reader or a
/// slice at a time.
struct Arrivals {
    now: i64,
    pieces: Vec<(i64, Vec<u8>)>,
    end_at: Option<i64>,
}

impl Arrivals {
    fn due(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        while self.pieces.first().is_some_and(|(at, _)| *at <= self.now) {
            out.extend(self.pieces.remove(0).1);
        }
        out
    }

    fn ended(&self) -> bool {
        self.pieces.is_empty() && self.end_at.is_some_and(|at| at <= self.now)
    }
}

impl Source for Arrivals {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        let until = self.now + wait_ms as i64;
        loop {
            let got = self.due();
            if !got.is_empty() {
                into[..got.len()].copy_from_slice(&got);
                return Ok(got.len());
            }
            if self.ended() || self.now >= until {
                return Ok(0);
            }
            self.now += 1;
        }
    }

    fn now_ms(&self) -> i64 {
        self.now
    }
}

fn sliced(mut a: Arrivals, done: fn(&[u8]) -> bool) -> Option<Vec<u8>> {
    let mut g = Gather::new(BOUNDS, a.now);
    loop {
        let got = a.due();
        match g.take(&got, a.ended(), a.now, done) {
            Gathered::More => a.now += TICK,
            Gathered::Whole(b) => return Some(b),
            Gathered::Nothing => return None,
        }
    }
}

fn arrivals(pieces: &[(i64, &[u8])], end_at: Option<i64>) -> Arrivals {
    Arrivals { now: 0, pieces: pieces.iter().map(|(t, b)| (*t, b.to_vec())).collect(), end_at }
}

/// Pieces arriving at their times, and when the far end finishes, if it does.
type Case<'a> = (&'a [(i64, &'a [u8])], Option<i64>);

#[test]
fn sliced_reading_takes_what_the_blocking_reader_takes() {
    let cases: [Case; 6] = [
        (&[(300, b"he"), (350, b"llo")], Some(360)),
        (&[(900, b"late")], None),
        (&[(1_200, b"too late")], None),
        (&[(100, b"a"), (250, b"b"), (800, b"c")], None),
        (&[], Some(50)),
        (&[(10, b"0123456789"), (20, b"0123456789")], None),
    ];
    for (pieces, end_at) in cases {
        let blocking = read_routed(&mut arrivals(pieces, end_at), BOUNDS, never);
        assert_eq!(sliced(arrivals(pieces, end_at), never), blocking, "{pieces:?}");
    }
}
