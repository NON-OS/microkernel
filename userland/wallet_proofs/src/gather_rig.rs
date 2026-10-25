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

//! The bounds, clock and arrivals the gathered-flight tests share.

use crate::wallet::net::routed_read::Bounds;
use crate::wallet::net::step::gather::{Gather, Gathered};

pub(super) const BOUNDS: Bounds =
    Bounds { first_ms: 1_000, quiet_ms: 200, total_ms: 5_000, max: 16 };

/* The window's tick between slices. */
pub(super) const TICK: i64 = 30;

pub(super) fn never(_: &[u8]) -> bool {
    false
}

/// Pieces that land at set times, read either by the blocking reader or a
/// slice at a time.
pub(super) struct Arrivals {
    pub(super) now: i64,
    pub(super) pieces: Vec<(i64, Vec<u8>)>,
    pub(super) end_at: Option<i64>,
}

impl Arrivals {
    pub(super) fn due(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        while self.pieces.first().is_some_and(|(at, _)| *at <= self.now) {
            out.extend(self.pieces.remove(0).1);
        }
        out
    }

    pub(super) fn ended(&self) -> bool {
        self.pieces.is_empty() && self.end_at.is_some_and(|at| at <= self.now)
    }
}

pub(super) fn sliced(mut a: Arrivals, done: fn(&[u8]) -> bool) -> Option<Vec<u8>> {
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

pub(super) fn arrivals(pieces: &[(i64, &[u8])], end_at: Option<i64>) -> Arrivals {
    Arrivals { now: 0, pieces: pieces.iter().map(|(t, b)| (*t, b.to_vec())).collect(), end_at }
}
