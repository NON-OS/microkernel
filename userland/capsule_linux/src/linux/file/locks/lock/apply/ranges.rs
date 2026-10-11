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

/* The table a lock leaves, its ranges split and merged, and its size. */

use super::super::table::Lock;
use super::take::met;
use alloc::vec::Vec;

/*
 * The table `all` becomes when `want` is applied. Pieces of an old lock
 * outside the range stay, as Linux splits a record lock.
 */
pub fn applied(all: Vec<Lock>, want: &Lock, add: bool) -> Vec<Lock> {
    let mut kept: Vec<Lock> = Vec::with_capacity(all.len() + 2);
    for l in all {
        if !met(&l, want) {
            kept.push(l);
            continue;
        }
        if l.start < want.start {
            kept.push(Lock { end: want.start, ..l.clone() });
        }
        if want.end < l.end {
            kept.push(Lock { start: want.end, ..l });
        }
    }
    if add {
        kept.push(want.clone());
    }
    kept
}

/* How many locks `applied` leaves, counted without making them. */
pub fn count_after(all: &[Lock], want: &Lock, add: bool) -> usize {
    let pieces = |l: &Lock| match met(l, want) {
        false => 1,
        true => usize::from(l.start < want.start) + usize::from(want.end < l.end),
    };
    all.iter().map(pieces).sum::<usize>() + usize::from(add)
}
