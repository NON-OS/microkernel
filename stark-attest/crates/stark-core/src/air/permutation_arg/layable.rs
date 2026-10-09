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

//! When laying a class onto a permutation is safe.
//!
//! Laying splices the class onto the image already there, so a class meeting
//! separate cycles merges them, which is what shared cells are for. Two cells of
//! one class already in the *same* cycle is the other case: the splice closes the
//! chain early and leaves a cell pointing at itself, so a binding that held before
//! is gone, quietly, while the product still closes and every other binding still
//! holds.
//!
//! Disjointness rules that out and rules out the merges too. This is the condition
//! that only rules out the split.

use super::cycles::Cell;
use alloc::vec::Vec;

fn find(p: &mut Vec<usize>, x: usize) -> usize {
    let mut r = x;
    while p[r] != r {
        r = p[r];
    }
    let mut c = x;
    while p[c] != c {
        let n = p[c];
        p[c] = r;
        c = n;
    }
    r
}

/// True when every class can be laid in order without splitting a cycle: at the
/// point each is laid, no two of its cells are already connected.
pub fn classes_are_layable(classes: &[Vec<Cell>]) -> bool {
    let mut ids: Vec<Cell> = classes.iter().flatten().copied().collect();
    ids.sort_unstable();
    ids.dedup();
    let at = |c: &Cell| ids.binary_search(c).unwrap();

    let mut p: Vec<usize> = (0..ids.len()).collect();
    for class in classes {
        let mut roots: Vec<usize> = Vec::with_capacity(class.len());
        for cell in class {
            let r = find(&mut p, at(cell));
            if roots.contains(&r) {
                return false;
            }
            roots.push(r);
        }
        for w in roots.windows(2) {
            let (a, b) = (find(&mut p, w[0]), find(&mut p, w[1]));
            p[a] = b;
        }
    }
    true
}
