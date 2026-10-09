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

use alloc::vec::Vec;

/// A trace cell, addressed the way the copy constraints name one.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

/// One permutation over the whole trace, built from equality classes rather than
/// one grand product per group. Each class becomes a cycle, so a satisfying
/// product forces every cell in it equal, which is what a per group argument did
/// separately.
pub struct WirePermutation {
    width: usize,
    rows: usize,
    sigma: Vec<usize>,
}

impl WirePermutation {
    pub fn identity(rows: usize, width: usize) -> WirePermutation {
        WirePermutation { width, rows, sigma: (0..rows * width).collect() }
    }

    fn index(&self, c: Cell) -> usize {
        c.row * self.width + c.col
    }

    /// Rotate the class into a cycle. Cells already carrying a non trivial image
    /// would silently drop their old class, so a caller must pass disjoint
    /// classes;  is what proves it did.
    pub fn add_class(&mut self, class: &[Cell]) {
        if class.len() < 2 {
            return;
        }
        let idx: Vec<usize> = class.iter().map(|c| self.index(*c)).collect();
        let first = self.sigma[idx[0]];
        for w in idx.windows(2) {
            self.sigma[w[0]] = self.sigma[w[1]];
        }
        let last = idx[idx.len() - 1];
        self.sigma[last] = first;
    }

    pub fn sigma(&self) -> &[usize] {
        &self.sigma
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
