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

/* A lock taken or dropped in the family's table, within its ceiling. */

use super::super::room::fits;
use super::super::table::{Lock, LOCKS};
use super::ranges::{applied, count_after};
use core::mem;

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
pub(super) fn met(l: &Lock, want: &Lock) -> bool {
    l.file == want.file && l.owner == want.owner && l.start < want.end && want.start < l.end
}

/* Drop every lock `gone` says has lost its owner. */
pub fn drop_where(gone: impl Fn(&Lock) -> bool) {
    LOCKS.0.borrow_mut().retain(|l| !gone(l));
}
