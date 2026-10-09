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

use super::cycles::Cell;
use alloc::vec::Vec;

/// Adding a class over a cell that already carries a non trivial image rewrites
/// its cycle and drops the earlier class, which unbinds it while every other
/// binding still holds. The per group form could not do this because each group
/// owned its own product; one permutation can, so the caller has to prove its
/// classes are disjoint before they are merged.
pub fn classes_are_disjoint(classes: &[Vec<Cell>]) -> bool {
    let mut seen: Vec<Cell> = classes.iter().flatten().copied().collect();
    let before = seen.len();
    seen.sort_unstable();
    seen.dedup();
    seen.len() == before
}
