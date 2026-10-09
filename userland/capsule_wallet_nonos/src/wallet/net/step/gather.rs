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

//! A TLS flight gathered a slice at a time. The same bounds `read_routed`
//! keeps, up to `first_ms` for anything, up to `quiet_ms` after each piece,
//! never more than `total_ms` in all, but held between slices instead of
//! waited out, so the window's thread reads what one slice brought, hands
//! it here, and goes back to painting. Pure: the clock is passed in, so
//! wallet_proofs drives it with a fake one.

use alloc::vec::Vec;

use super::super::bounds::Bounds;

/// Where a gathered flight stands after a slice.
#[derive(Debug, PartialEq, Eq)]
pub enum Gathered {
    /// Not whole and still inside its bounds: read again on a later slice.
    More,
    /// What came, judged as it is.
    Whole(Vec<u8>),
    /// Nothing came in time, or more than a flight may hold.
    Nothing,
}

pub struct Gather {
    bounds: Bounds,
    start: i64,
    /// When the last piece came, or the start before any did.
    last: i64,
    out: Vec<u8>,
}

impl Gather {
    pub fn new(bounds: Bounds, now: i64) -> Gather {
        Gather { bounds, start: now, last: now, out: Vec::new() }
    }

    /// What one slice brought, and whether the far end said it finished,
    /// at `now`. `done` says the flight is whole.
    pub fn take(
        &mut self,
        bytes: &[u8],
        ended: bool,
        now: i64,
        mut done: impl FnMut(&[u8]) -> bool,
    ) -> Gathered {
        if !bytes.is_empty() {
            self.out.extend_from_slice(bytes);
            self.last = now;
            if self.out.len() > self.bounds.max {
                self.out.clear();
                return Gathered::Nothing;
            }
            if done(&self.out) {
                return self.finish();
            }
        }
        if ended {
            return self.finish();
        }
        let wait = if self.out.is_empty() { self.bounds.first_ms } else { self.bounds.quiet_ms };
        let quiet = now.saturating_sub(self.last) >= clamp(wait);
        let spent = now.saturating_sub(self.start) >= clamp(self.bounds.total_ms);
        if quiet || spent {
            return self.finish();
        }
        Gathered::More
    }

    /// The stream broke: what came before the break is judged as it is.
    pub fn broke(&mut self) -> Gathered {
        self.finish()
    }

    fn finish(&mut self) -> Gathered {
        let out = core::mem::take(&mut self.out);
        if out.is_empty() {
            Gathered::Nothing
        } else {
            Gathered::Whole(out)
        }
    }
}

fn clamp(ms: u64) -> i64 {
    i64::try_from(ms).unwrap_or(i64::MAX)
}
