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


//! Walking a stage-three pair back down to its eight indices, putting each
//! level in canonical order on the way up.

use super::heap::{invert_bucket, item_bucket, item_left, item_right, slot};
use super::Solver;
use crate::solution::{tree_idx2, tree_idx4, Solution};

impl Solver {
    fn build1(&self, out: &mut [u16], root: u32) {
        let bucket = item_bucket(root);
        out[0] = self.s1_idx[slot(bucket, item_left(root))];
        out[1] = self.s1_idx[slot(invert_bucket(bucket), item_right(root))];
        if out[0] > out[1] {
            out.swap(0, 1);
        }
    }

    fn build2(&self, out: &mut [u16], root: u32) {
        let bucket = item_bucket(root);
        let left = self.s2_idx[slot(bucket, item_left(root))];
        let right = self.s2_idx[slot(invert_bucket(bucket), item_right(root))];
        self.build1(&mut out[0..2], left);
        self.build1(&mut out[2..4], right);
        if tree_idx2(&out[0..2]) > tree_idx2(&out[2..4]) {
            out.swap(0, 2);
            out.swap(1, 3);
        }
    }

    pub(super) fn build(&self, left: u32, right: u32) -> Solution {
        let mut s = Solution::default();
        self.build2(&mut s.idx[0..4], left);
        self.build2(&mut s.idx[4..8], right);
        if tree_idx4(&s.idx[0..4]) > tree_idx4(&s.idx[4..8]) {
            for i in 0..4 {
                s.idx.swap(i, i + 4);
            }
        }
        s
    }
}
