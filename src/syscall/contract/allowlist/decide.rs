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

/*
 * The per-syscall decision. Pure: no globals, no locks, no clock; the list
 * and the caller's launcher standing are all it reads. A number with no
 * registry slot is refused, as it is before the list is consulted.
 */

use super::set::Allowlist;
use crate::syscall::abi::index::slot_of;

#[inline]
pub fn allowed(list: &Allowlist, nr: u64, caller_launches: bool) -> bool {
    match slot_of(nr) {
        Some(slot) if caller_launches => list.launcher(slot),
        Some(slot) => list.plain(slot),
        None => false,
    }
}

/// Whether the answer for `nr` turns on the caller's launcher standing, so
/// the dispatcher asks that question only when it matters.
#[inline]
pub fn depends_on_launcher(list: &Allowlist, nr: u64) -> bool {
    match slot_of(nr) {
        Some(slot) => list.plain(slot) != list.launcher(slot),
        None => false,
    }
}

/*
 * The dispatcher's form: one slot lookup, and the launcher question asked
 * only when the two answers differ.
 */
#[inline]
pub fn decide(list: &Allowlist, nr: u64, caller_launches: fn() -> bool) -> bool {
    let Some(slot) = slot_of(nr) else { return false };
    let (plain, launcher) = (list.plain(slot), list.launcher(slot));
    if plain == launcher {
        plain
    } else if caller_launches() {
        launcher
    } else {
        plain
    }
}
