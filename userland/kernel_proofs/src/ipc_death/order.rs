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

//! The order the kernel keeps: the names go before the calls are cleared,
//! gone is recorded under the queue lock before any caller is woken, a push
//! asks gone under that lock, and the wait looks for its token before it
//! sleeps, so a caller already waiting returns at once.

const TEARDOWN: &str = include_str!("../../../../src/process/exit/teardown.rs");
const CLEAR: &str =
    include_str!("../../../../src/syscall/microkernel/ipc/pending_reply/clear_pid.rs");
const PUSH: &str = include_str!("../../../../src/syscall/microkernel/ipc/pending_reply/push.rs");
const RECV: &str = include_str!("../../../../src/syscall/microkernel/ipc/recv_reply.rs");
const CALL: &str = include_str!("../../../../src/syscall/microkernel/ipc/call/sys_ipc_call.rs");

fn at(src: &str, what: &str) -> usize {
    src.find(what).unwrap_or_else(|| panic!("`{what}` is gone"))
}

#[test]
fn the_names_go_before_the_calls_are_cleared() {
    let names = at(TEARDOWN, "super::finalize::release_names(pid, &pcb);");
    let clear = at(TEARDOWN, "ipc::release_pending_replies_for_pid(pid);");
    assert!(names < clear);
    assert_eq!(TEARDOWN.matches("release_pending_replies_for_pid(").count(), 1);
}

#[test]
fn gone_is_recorded_under_the_lock_before_any_caller_wakes() {
    let lock = at(CLEAR, "let mut queues = PENDING.lock();");
    let record = at(CLEAR, "super::abandoned::record(pid, &owed);");
    let unlock = at(CLEAR, "drop(queues);");
    let wake = at(CLEAR, "crate::sched::wake_process(caller);");
    assert!(lock < record && record < unlock && unlock < wake);
}

#[test]
fn a_push_to_a_gone_server_is_refused_with_esrch() {
    let lock = at(PUSH, "let mut map = PENDING.lock();");
    let gone = at(PUSH, "if super::abandoned::gone(server_pid) {");
    let queued = at(PUSH, "queue.push_back(");
    assert!(lock < gone && gone < queued);
    assert!(CALL.contains("Err(pending_reply::Refused::Gone) => return ERRNO_SRCH,"));
}

#[test]
fn the_wait_looks_for_its_token_before_it_sleeps() {
    let check = at(RECV, "if super::pending_reply::abandoned(pid, want) {");
    let answer = at(RECV, "return ERRNO_SRCH;");
    let sleep = at(RECV, "crate::sched::sleep_until_unless_woken(");
    assert!(check < answer && answer < sleep);
}
