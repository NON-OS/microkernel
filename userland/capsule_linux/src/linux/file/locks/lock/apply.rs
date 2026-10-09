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

/* A lock taken or dropped, splitting and merging the ranges it meets. */

use alloc::vec::Vec;
use core::mem;

use super::room::fits;
use super::table::{Lock, LOCKS};

/*
 * Clear `want`'s owner from `want`'s range of the file, then, if `add`,
 * hold that range as `want` says; ENOLCK, changing nothing, when that would
 * grow the table past its ceiling (`room`).
 */
pub fn apply(want: &Lock, add: bool) -> Result<(), i64> {
    let mut all = LOCKS.0.borrow_mut();
    fits(all.len(), count_after(&all, want, add))?;
    let old = mem::take(&mut *all);
    *all = applied(old, want, add);
    Ok(())
}

/* Whether `l` is `want`'s owner's lock on `want`'s file, meeting its range. */
fn met(l: &Lock, want: &Lock) -> bool {
    l.file == want.file && l.owner == want.owner && l.start < want.end && want.start < l.end
}

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

/* Drop every lock `gone` says has lost its owner. */
pub fn drop_where(gone: impl Fn(&Lock) -> bool) {
    LOCKS.0.borrow_mut().retain(|l| !gone(l));
}
