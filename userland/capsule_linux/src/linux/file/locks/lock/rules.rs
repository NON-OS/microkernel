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

/* Linux's rules for when two locks meet. */

use super::apply::apply;
use super::table::{record, Lock, LOCKS};

/* Whether a lock held by `held` stands in the way of one `want` asks for. */
fn conflicts(held: &Lock, want: &Lock) -> bool {
    held.file == want.file
        && held.owner != want.owner
        && record(held.owner) == record(want.owner)
        && (held.write || want.write)
        && held.start < want.end
        && want.start < held.end
}

/* The first lock in the way of `want`, as F_GETLK reports it. */
pub fn blocker(want: &Lock) -> Option<Lock> {
    LOCKS.0.borrow().iter().find(|l| conflicts(l, want)).cloned()
}

/* Why a lock was not taken. */
pub enum Refused {
    /* Another owner's lock is in the way: a caller may wait for it. */
    Blocked,
    /* The table is full: ENOLCK, which no wait changes. */
    Full,
}

/*
 * Take `want`, or say what refused it. An unlock is a `want` that `apply`
 * is told to only remove.
 */
pub fn take(want: Lock) -> Result<(), Refused> {
    if blocker(&want).is_some() {
        return Err(Refused::Blocked);
    }
    apply(&want, true).map_err(|_| Refused::Full)
}
