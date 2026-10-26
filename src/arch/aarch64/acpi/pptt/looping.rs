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

//! Whether following one reference from each entry ever comes back round:
//! each entry is visited once, so a long chain costs its length, once.

use alloc::vec;

/// The first entry from which `next` loops back onto its own path.
pub(crate) fn looping(len: usize, next: impl Fn(usize) -> Option<usize>) -> Option<usize> {
    let mut state = vec![0u8; len]; // 0 unseen, 1 on the current path, 2 known to end
    for start in 0..len {
        let (mut path, mut at) = (vec![], Some(start));
        while let Some(i) = at {
            match state[i] {
                2 => break,
                1 => return Some(start),
                _ => {
                    state[i] = 1;
                    path.push(i);
                    at = next(i);
                }
            }
        }
        path.into_iter().for_each(|i| state[i] = 2);
    }
    None
}
