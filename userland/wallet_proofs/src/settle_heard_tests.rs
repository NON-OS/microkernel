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

//! The wallet's ASYNC answers from settle (v1.4.53): only settle's registry
//! pid, on its wire, answering the request outstanding by op and id, is
//! believed; anything else, a late answer included, is dropped unread.

use nonos_settle_proto::{reply, request_async, FLAG_ASYNC, OP_INTENT, OP_READ};

use crate::settle_heard::{expired, heard, Asked, Heard, STEP_MS};

const SETTLE: u32 = 42;
const ASKED: Asked = Asked { op: OP_READ, id: 7, settle_pid: SETTLE, at_ms: 1_000 };

#[test]
fn the_answer_to_the_request_outstanding_is_believed() {
    let rx = reply(OP_READ, FLAG_ASYNC, 7, 0, &[1, 2]).unwrap();
    assert_eq!(heard(Some(&ASKED), SETTLE, &rx), Heard::Answer(Ok(vec![1, 2])));
    let refused = reply(OP_READ, FLAG_ASYNC, 7, -22, &[]).unwrap();
    assert_eq!(heard(Some(&ASKED), SETTLE, &refused), Heard::Answer(Err(-22)));
}

#[test]
fn another_pid_a_wrong_id_or_op_or_a_late_answer_is_dropped() {
    let rx = reply(OP_READ, FLAG_ASYNC, 7, 0, &[1]).unwrap();
    assert_eq!(heard(Some(&ASKED), 99, &rx), Heard::Dropped, "another pid");
    assert_eq!(heard(Some(&ASKED), 0, &rx), Heard::Dropped, "no sender");
    let other_id = reply(OP_READ, FLAG_ASYNC, 8, 0, &[1]).unwrap();
    assert_eq!(heard(Some(&ASKED), SETTLE, &other_id), Heard::Dropped, "a wrong id");
    let other_op = reply(OP_INTENT, FLAG_ASYNC, 7, 0, &[1]).unwrap();
    assert_eq!(heard(Some(&ASKED), SETTLE, &other_op), Heard::Dropped, "a wrong op");
    assert_eq!(heard(None, SETTLE, &rx), Heard::Dropped, "late: nothing outstanding");
    let mut magic = rx.clone();
    magic[0] ^= 1;
    assert_eq!(heard(Some(&ASKED), SETTLE, &magic), Heard::Dropped, "not NSTL");
    assert_eq!(heard(Some(&ASKED), SETTLE, &rx[..rx.len() - 1]), Heard::Dropped, "cut");
    assert_eq!(heard(Some(&ASKED), SETTLE, &[]), Heard::Dropped, "empty");
    let request = request_async(OP_READ, 7, &[1]).unwrap();
    assert_eq!(heard(Some(&ASKED), SETTLE, &request), Heard::Dropped, "our own request echoed");
}

#[test]
fn a_request_expires_at_step_ms() {
    assert!(!expired(&ASKED, 1_000 + STEP_MS - 1));
    assert!(expired(&ASKED, 1_000 + STEP_MS));
    assert!(!expired(&ASKED, 0), "a clock behind the send never wraps");
    assert_eq!(STEP_MS, 150_000);
}
