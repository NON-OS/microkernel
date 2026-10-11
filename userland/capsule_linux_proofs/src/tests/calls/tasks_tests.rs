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

//! RLIMIT_NPROC for a family. Every task is a kernel process made for this
//! capsule and a record in its heap, so a fork bomb is held where Linux
//! holds one: fork and clone are EAGAIN at the limit, threads and zombies
//! counted with the processes.

use crate::linux::abi::errno::EAGAIN;
use crate::linux::call::tasks::{count, room, MAX_TASKS};

/// One process of the family: its other threads and its unwaited children.
#[derive(Clone, Copy)]
struct Proc {
    threads: usize,
    zombies: usize,
}

fn tasks(family: &[Proc]) -> usize {
    count(family.iter().map(|p| (p.threads, p.zombies)))
}

/// `:(){ :|:& };:`: every process forks twice, every round, forever. The
/// family stops at the limit and every later fork is EAGAIN.
#[test]
fn a_fork_bomb_stops_at_the_limit() {
    let mut family = vec![Proc { threads: 0, zombies: 0 }];
    let mut refused = 0;
    for _round in 0..32 {
        for _ in 0..family.len() * 2 {
            match room(tasks(&family)) {
                Ok(()) => family.push(Proc { threads: 0, zombies: 0 }),
                Err(e) => {
                    assert_eq!(e, EAGAIN);
                    refused += 1;
                }
            }
            assert!(family.len() <= MAX_TASKS, "past the limit");
        }
    }
    assert_eq!(tasks(&family), MAX_TASKS);
    assert!(refused > 0);
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
