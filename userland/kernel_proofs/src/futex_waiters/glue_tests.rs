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

//! The glue the killed-waiter model stands for, read (L77e): teardown sweeps
//! the thread off every futex once it is a zombie, a wait that ended before
//! it queued leaves at once, and a wake takes live waiters only.

const TEARDOWN: &str = include_str!("../../../../src/process/exit/teardown.rs");
const FORGET: &str = include_str!("../../../../src/process/exit/forget.rs");
const WAIT: &str = include_str!("../../../../src/syscall/microkernel/futex/wait.rs");
const WAKE: &str = include_str!("../../../../src/syscall/microkernel/futex/wake.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn a_killed_waiter_is_swept_and_a_wake_takes_the_living() {
    let zombie = at(TEARDOWN, "= ProcessState::Zombie(exit_code);");
    assert!(zombie < at(TEARDOWN, "super::forget::forget(pid);"), "swept before it ended");
    assert!(FORGET.contains("futex::forget(pid);"), "the futex sweep left out");
    let joined = &WAIT[at(WAIT, "join(&mut FUTEX_QUEUE.lock(), key, caller);")..];
    let ask = at(joined, "super::queue::ended(caller)");
    assert!(ask < at(joined, "sleep_until_unless_woken("), "asked after the sleep");
    assert!(joined[ask..].contains("if dead ||"), "the answer ignored");
    assert!(WAKE.contains("take_live(v, count as usize, ended)"), "a wake counts the dead");
}
