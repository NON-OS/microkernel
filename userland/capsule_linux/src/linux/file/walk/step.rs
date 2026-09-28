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

use super::path::made_link;

const MAX_HOPS: usize = 16;

/*
 * `path` with every link in it followed; its last component too when
 * `last` is set.
 */
pub fn follow(guest: &Guest, path: Vec<u8>, last: bool) -> Vec<u8> {
    let mut path = guest.links.follow(path, last);
    for _ in 0..MAX_HOPS {
        match made_link(&path, last) {
            Some(next) => path = guest.links.follow(next, last),
            None => break,
        }
    }
    path
}
