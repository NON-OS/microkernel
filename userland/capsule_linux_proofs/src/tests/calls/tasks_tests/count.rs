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

//! How the family's tasks are counted, and where the count stops.

use crate::linux::abi::errno::EAGAIN;
use crate::linux::call::tasks::{count, room, MAX_TASKS};

/// One process of the family: its other threads and its unwaited children.
#[derive(Clone, Copy)]
pub(super) struct Proc {
    pub(super) threads: usize,
    pub(super) zombies: usize,
}

pub(super) fn tasks(family: &[Proc]) -> usize {
    count(family.iter().map(|p| (p.threads, p.zombies)))
}

/// Threads made by clone count, and a child that has ended but was never
/// waited for still holds its place, as a zombie does on Linux.
#[test]
fn threads_and_zombies_count() {
    let family = [Proc { threads: 300, zombies: 0 }, Proc { threads: 0, zombies: 210 }];
    assert_eq!(tasks(&family), 512);
    assert_eq!(room(tasks(&family)), Err(EAGAIN));
    let waited = [Proc { threads: 300, zombies: 0 }, Proc { threads: 0, zombies: 209 }];
    assert_eq!(room(tasks(&waited)), Ok(()), "a wait gives a place back");
}

#[test]
fn the_limit_is_exact_and_the_count_saturates() {
    assert_eq!(room(MAX_TASKS - 1), Ok(()));
    assert_eq!(room(MAX_TASKS), Err(EAGAIN));
    assert_eq!(room(usize::MAX), Err(EAGAIN));
    assert_eq!(count([(usize::MAX, usize::MAX)].into_iter()), usize::MAX);
    assert_eq!(count(core::iter::empty()), 0);
}
