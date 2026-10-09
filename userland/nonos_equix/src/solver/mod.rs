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


//! The Equi-X solver: hash all 65536 indices into buckets, then pair up
//! complementary buckets three times over until eight hashes sum to zero.
//!
//! Hashing is by far the longest part, so it is split out: `hash_some` takes
//! a budget and can be called once per tick, which keeps the capsule's event
//! loop answering while a puzzle is being solved. The pairing stages after it
//! take a few milliseconds and run in one go.
//!
//! Bucket sizes, the order buckets are visited in, and the order pairs are
//! recorded in all follow the reference, so the solutions come out in the
//! same order it gives them. That order decides which solution a client
//! submits when several pass the effort check.

mod build;
mod heap;
mod stages;

use alloc::vec::Vec;

use crate::hashx::HashX;
use crate::solution::Solution;

pub const INDEX_SPACE: u32 = 1 << 16;
pub const MAX_SOLS: usize = 8;
const COARSE_BUCKETS: usize = 256;
const FINE_BUCKETS: usize = 128;
const COARSE_ITEMS: usize = 336;
const FINE_ITEMS: usize = 12;
const SLOTS: usize = COARSE_BUCKETS * COARSE_ITEMS;

/// The solver's working memory, about 1.9 MB, kept across solves so a run of
/// nonces allocates once.
pub struct Solver {
    s1_counts: [u16; COARSE_BUCKETS],
    s1_idx: Vec<u16>,
    s2_counts: [u16; COARSE_BUCKETS],
    s2_idx: Vec<u32>,
    s2_data: Vec<u64>,
    /// Stage one's data until stage one is done, then stage three's index in
    /// the low half of each word and its data in the high half. The reference
    /// shares this memory through a union; one word holding both halves does
    /// the same without reinterpreting anything.
    shared: Vec<u64>,
    s3_counts: [u16; COARSE_BUCKETS],
    scratch_counts: [u8; FINE_BUCKETS],
    scratch: [[u16; FINE_ITEMS]; FINE_BUCKETS],
    next: u32,
}

fn zeroed<T: Copy + Default>(n: usize) -> Option<Vec<T>> {
    let mut v = Vec::new();
    v.try_reserve_exact(n).ok()?;
    v.resize(n, T::default());
    Some(v)
}

impl Solver {
    /// A solver, or `None` when the heap cannot give it its memory. Refusing
    /// here is what lets a caller fail one introduction instead of aborting
    /// the capsule.
    pub fn new() -> Option<Self> {
        Some(Self {
            s1_counts: [0; COARSE_BUCKETS],
            s1_idx: zeroed(SLOTS)?,
            s2_counts: [0; COARSE_BUCKETS],
            s2_idx: zeroed(SLOTS)?,
            s2_data: zeroed(SLOTS)?,
            shared: zeroed(SLOTS)?,
            s3_counts: [0; COARSE_BUCKETS],
            scratch_counts: [0; FINE_BUCKETS],
            scratch: [[0; FINE_ITEMS]; FINE_BUCKETS],
            next: 0,
        })
    }

    /// Begins hashing for a new challenge.
    pub fn start(&mut self) {
        self.s1_counts = [0; COARSE_BUCKETS];
        self.next = 0;
    }

    /// Hashes up to `budget` more indices. True once every index is hashed.
    pub fn hash_some(&mut self, hashx: &HashX, budget: u32) -> bool {
        let end = self.next.saturating_add(budget).min(INDEX_SPACE);
        for i in self.next..end {
            self.stage0_one(hashx, i);
        }
        self.next = end;
        end == INDEX_SPACE
    }

    /// Runs the pairing stages and writes the solutions found, at most eight,
    /// into the front of `out`. Returns how many. Call only after
    /// `hash_some` has returned true.
    pub fn finish(&mut self, out: &mut [Solution; MAX_SOLS]) -> usize {
        if self.next != INDEX_SPACE {
            return 0;
        }
        self.stage1();
        self.stage2();
        self.stage3(out)
    }

    /// Every solution to one challenge, in one call.
    pub fn solve(&mut self, hashx: &HashX, out: &mut [Solution; MAX_SOLS]) -> usize {
        self.start();
        self.hash_some(hashx, INDEX_SPACE);
        self.finish(out)
    }
}
