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

//! A fork bomb held at the limit, as Linux holds one.

use super::count::{tasks, Proc};
use crate::linux::abi::errno::EAGAIN;
use crate::linux::call::tasks::{room, MAX_TASKS};

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
