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

/* What a walk's caller is told at each step, and the walk with no limit. */

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::path::walk;

/* Linux's MAXSYMLINKS: more links than this in one walk is a loop. */
pub(super) const MAX_HOPS: usize = 40;

/* One step of a walk, for a caller that limits where a walk may go. */
pub enum Step<'a> {
    /* The walk stands at this path. */
    At(&'a [u8]),
    /* The walk is about to follow the link at `at`, which leads to `to`. */
    Link { at: &'a [u8], to: &'a [u8] },
}

/*
 * `path` with every link in it followed, its last name too when `last` is
 * set, and every `.` and `..` resolved. A trailing slash asks for the last
 * name to be followed, as it does on Linux.
 */
pub fn follow(guest: &Guest, path: Vec<u8>, last: bool) -> Vec<u8> {
    /* With no check to refuse a step, the walk always ends somewhere. */
    walk(guest, path, last, |_| Ok(())).unwrap_or_else(|_| alloc::vec![b'/'])
}
