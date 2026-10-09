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

//! Where the n-th block of a file hangs in its index.

use super::file_consts::{DIRECT_SLOTS, FANOUT, LEVELS};

/// The root slot holding block `n`, how many pointer blocks lie between that
/// slot and the data, and the index taken in each, the top level first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Place {
    pub slot: usize,
    pub depth: usize,
    pub path: [usize; LEVELS],
}

/// Place block `n`, or None past the largest file the index can describe.
pub(crate) fn locate(n: u64) -> Option<Place> {
    let mut path = [0usize; LEVELS];
    if n < DIRECT_SLOTS as u64 {
        return Some(Place { slot: n as usize, depth: 0, path });
    }
    let mut rest = n - DIRECT_SLOTS as u64;
    let mut span = 1u64;
    for depth in 1..=LEVELS {
        span *= FANOUT as u64;
        if rest < span {
            /*
             * `rest` written in base FANOUT with `depth` digits, the most
             * significant first: the top pointer block's index leads.
             */
            let mut digits = rest;
            for level in (0..depth).rev() {
                path[level] = (digits % FANOUT as u64) as usize;
                digits /= FANOUT as u64;
            }
            return Some(Place { slot: DIRECT_SLOTS + depth - 1, depth, path });
        }
        rest -= span;
    }
    None
}
