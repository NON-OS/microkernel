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

//! What the kernel hands the rule, read from its source: settle asks it
//! once, with whether the receive timed out, the caller's own bound and how
//! long the call waited, so a call that failed at once is never late.

const SETTLE: &str = include_str!("../../../../src/syscall/microkernel/ipc/call/settle.rs");
const CALL: &str = include_str!("../../../../src/syscall/microkernel/ipc/call/sys_ipc_call.rs");

#[test]
fn settle_asks_the_rule_once_with_whether_the_receive_timed_out() {
    assert_eq!(SETTLE.matches("no_answer(").count(), 1, "settle asks the rule once");
    assert!(SETTLE.contains("no_answer(rc == ERRNO_TIMEDOUT, waited.0, DEFAULT_MS, waited.1)"));
}

#[test]
fn the_call_hands_settle_its_own_bound_and_the_time_it_waited() {
    let waited = "let waited = (timeout_ms, crate::time::timestamp_millis().saturating_sub(start));";
    assert!(CALL.contains(waited));
    assert!(CALL.contains("settle(endpoint.as_ref(), pid, (req, req_len, resp), recv_result, waited);"));
}
