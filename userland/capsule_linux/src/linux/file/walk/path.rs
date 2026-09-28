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

/* Following a path, a name at a time, under a root. */

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::link::link_at;
use super::state::{joined, names, Walk};
use super::step::{Step, MAX_HOPS};

/*
 * The walk `follow` makes, with `check` asked about each step first; its
 * refusal ends the walk.
 */
pub fn walk(
    guest: &Guest,
    path: Vec<u8>,
    last: bool,
    mut check: impl FnMut(Step) -> Result<(), i64>,
) -> Result<Vec<u8>, i64> {
    let last = last || path.last() == Some(&b'/');
    let mut w = Walk { todo: names(&path).collect(), done: Vec::new(), hops: 0 };
    while let Some(name) = w.todo.pop_front() {
        if name == b".." {
            w.up(&path);
            check(Step::At(&joined(&w.done)))?;
            continue;
        }
        w.done.push(name);
        let at = joined(&w.done);
        if w.todo.is_empty() && !last {
            check(Step::At(&at))?;
            break;
        }
        let Some(to) = link_at(guest, &at).filter(|_| w.hops < MAX_HOPS) else {
            check(Step::At(&at))?;
            continue;
        };
        check(Step::Link { at: &at, to: &to })?;
        w.into_link(&to);
    }
    Ok(joined(&w.done))
}
