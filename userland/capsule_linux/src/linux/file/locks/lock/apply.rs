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

use super::table::{Lock, LOCKS};

/*
 * Clear `want`'s owner from `want`'s range of the file, then, if `add`,
 * hold that range as `want` says. Pieces of an old lock outside the range
 * stay, as Linux splits a record lock.
 */
pub fn apply(want: &Lock, add: bool) {
    let mut all = LOCKS.0.borrow_mut();
    let mut kept: Vec<Lock> = Vec::with_capacity(all.len() + 2);
    for l in all.drain(..) {
        let mine = l.file == want.file && l.owner == want.owner;
        if !mine || l.end <= want.start || want.end <= l.start {
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
    *all = kept;
}

/* Drop every lock `gone` says has lost its owner. */
pub fn drop_where(gone: impl Fn(&Lock) -> bool) {
    LOCKS.0.borrow_mut().retain(|l| !gone(l));
}
