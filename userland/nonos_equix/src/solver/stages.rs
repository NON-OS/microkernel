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


//! The four stages. Each pairing stage walks buckets 0 through 128 against
//! their complements, files the complement bucket's items into a small hash
//! table by their low 7 bits, and pairs each item of the bucket with the
//! entries that complete it. Bucket 0 and bucket 128 are their own
//! complements; for them filing and pairing interleave, item by item, as the
//! reference does.
//!
//! A value from any bucket but 0 carries one into its sum: the bucket index
//! was the value's low 8 bits, and the pair's low bits add to exactly 256.

use super::heap::{invert_bucket, invert_scratch, make_item, slot};
use super::{Solver, COARSE_BUCKETS, COARSE_ITEMS, FINE_BUCKETS, FINE_ITEMS, MAX_SOLS};
use crate::hashx::HashX;
use crate::solution::{Solution, STAGE1_MASK};

const BUCK_END: usize = COARSE_BUCKETS / 2 + 1;

impl Solver {
    pub(super) fn stage0_one(&mut self, hashx: &HashX, i: u32) {
        let value = hashx.hash(u64::from(i));
        let bucket = (value % COARSE_BUCKETS as u64) as usize;
        let pos = usize::from(self.s1_counts[bucket]);
        if pos >= COARSE_ITEMS {
            return;
        }
        self.s1_counts[bucket] = pos as u16 + 1;
        self.s1_idx[slot(bucket, pos)] = i as u16;
        self.shared[slot(bucket, pos)] = value / COARSE_BUCKETS as u64;
    }

    /// Files one item of the complement bucket into the scratch table.
    /// False when its fine bucket is full, in which case it is dropped.
    fn file(&mut self, value: u64, pos: usize) -> bool {
        let fine = (value % FINE_BUCKETS as u64) as usize;
        let at = usize::from(self.scratch_counts[fine]);
        if at >= FINE_ITEMS {
            return false;
        }
        self.scratch_counts[fine] = at as u8 + 1;
        self.scratch[fine][at] = pos as u16;
        true
    }

    pub(super) fn stage1(&mut self) {
        self.s2_counts = [0; COARSE_BUCKETS];
        for bucket in 0..BUCK_END {
            let cpl = invert_bucket(bucket);
            self.scratch_counts = [0; FINE_BUCKETS];
            for pos in 0..usize::from(self.s1_counts[cpl]) {
                if self.file(self.shared[slot(cpl, pos)], pos) && cpl == bucket {
                    self.pairs1(bucket, pos, cpl);
                }
            }
            if cpl != bucket {
                for pos in 0..usize::from(self.s1_counts[bucket]) {
                    self.pairs1(bucket, pos, cpl);
                }
            }
        }
    }

    fn pairs1(&mut self, bucket: usize, pos: usize, cpl: usize) {
        let value = self.shared[slot(bucket, pos)] + u64::from(bucket != 0);
        let fine_cpl = invert_scratch((value % FINE_BUCKETS as u64) as usize);
        for f in 0..usize::from(self.scratch_counts[fine_cpl]) {
            let cpl_pos = usize::from(self.scratch[fine_cpl][f]);
            let sum = (value + self.shared[slot(cpl, cpl_pos)]) / FINE_BUCKETS as u64;
            let to = (sum % COARSE_BUCKETS as u64) as usize;
            let at = usize::from(self.s2_counts[to]);
            if at >= COARSE_ITEMS {
                continue;
            }
            self.s2_counts[to] = at as u16 + 1;
            self.s2_idx[slot(to, at)] = make_item(bucket, pos, cpl_pos);
            self.s2_data[slot(to, at)] = sum / COARSE_BUCKETS as u64;
        }
    }

    pub(super) fn stage2(&mut self) {
        self.s3_counts = [0; COARSE_BUCKETS];
        for bucket in 0..BUCK_END {
            let cpl = invert_bucket(bucket);
            self.scratch_counts = [0; FINE_BUCKETS];
            for pos in 0..usize::from(self.s2_counts[cpl]) {
                if self.file(self.s2_data[slot(cpl, pos)], pos) && cpl == bucket {
                    self.pairs2(bucket, pos, cpl);
                }
            }
            if cpl != bucket {
                for pos in 0..usize::from(self.s2_counts[bucket]) {
                    self.pairs2(bucket, pos, cpl);
                }
            }
        }
    }

    fn pairs2(&mut self, bucket: usize, pos: usize, cpl: usize) {
        let value = self.s2_data[slot(bucket, pos)] + u64::from(bucket != 0);
        let fine_cpl = invert_scratch((value % FINE_BUCKETS as u64) as usize);
        for f in 0..usize::from(self.scratch_counts[fine_cpl]) {
            let cpl_pos = usize::from(self.scratch[fine_cpl][f]);
            let sum = (value + self.s2_data[slot(cpl, cpl_pos)]) / FINE_BUCKETS as u64;
            let to = (sum % COARSE_BUCKETS as u64) as usize;
            let at = usize::from(self.s3_counts[to]);
            if at >= COARSE_ITEMS {
                continue;
            }
            self.s3_counts[to] = at as u16 + 1;
            let data = (sum / COARSE_BUCKETS as u64) as u32;
            let item = make_item(bucket, pos, cpl_pos);
            self.shared[slot(to, at)] = u64::from(data) << 32 | u64::from(item);
        }
    }

    pub(super) fn s3_idx(&self, bucket: usize, pos: usize) -> u32 {
        self.shared[slot(bucket, pos)] as u32
    }

    fn s3_data(&self, bucket: usize, pos: usize) -> u32 {
        (self.shared[slot(bucket, pos)] >> 32) as u32
    }

    pub(super) fn stage3(&mut self, out: &mut [Solution; MAX_SOLS]) -> usize {
        let mut found = 0usize;
        for bucket in 0..BUCK_END {
            let cpl = invert_bucket(bucket);
            self.scratch_counts = [0; FINE_BUCKETS];
            for pos in 0..usize::from(self.s3_counts[cpl]) {
                if self.file(u64::from(self.s3_data(cpl, pos)), pos)
                    && cpl == bucket
                    && self.pairs3(bucket, pos, cpl, out, &mut found)
                {
                    return found;
                }
            }
            if cpl != bucket {
                for pos in 0..usize::from(self.s3_counts[bucket]) {
                    if self.pairs3(bucket, pos, cpl, out, &mut found) {
                        return found;
                    }
                }
            }
        }
        found
    }

    /// Records every solution this item completes. True once the output is
    /// full, which ends the search.
    fn pairs3(&self, bucket: usize, pos: usize, cpl: usize, out: &mut [Solution; MAX_SOLS], found: &mut usize) -> bool {
        let value = self.s3_data(bucket, pos).wrapping_add(u32::from(bucket != 0));
        let fine_cpl = invert_scratch((value % FINE_BUCKETS as u32) as usize);
        for f in 0..usize::from(self.scratch_counts[fine_cpl]) {
            let cpl_pos = usize::from(self.scratch[fine_cpl][f]);
            let sum = value.wrapping_add(self.s3_data(cpl, cpl_pos)) / FINE_BUCKETS as u32;
            if u64::from(sum) & STAGE1_MASK == 0 {
                out[*found] = self.build(self.s3_idx(bucket, pos), self.s3_idx(cpl, cpl_pos));
                *found += 1;
                if *found >= MAX_SOLS {
                    return true;
                }
            }
        }
        false
    }
}
