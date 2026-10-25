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

//! The attest service's board, asked once.

use core::ptr;
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};
use nonos_route_proof::{
    ask_frame, read_answer, reply_body, Latest, ANSWER_FRAME_LEN, ATTEST_SERVICE, OP_PROOF_ROUTE,
};

const BOARD_REPLY_MS: u64 = 100;

pub(super) fn board() -> Option<(Latest, Latest)> {
    let mut port = 0u32;
    let rc = mk_service_lookup(
        ATTEST_SERVICE.as_ptr(),
        ATTEST_SERVICE.len(),
        &mut port as *mut u32,
        ptr::null_mut(),
    );
    if rc != 0 || port == 0 {
        return None;
    }
    let ask = ask_frame(1);
    let mut reply = [0u8; ANSWER_FRAME_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        ask.as_ptr(),
        ask.len(),
        reply.as_mut_ptr(),
        reply.len(),
        BOARD_REPLY_MS,
    );
    if n <= 0 {
        return None;
    }
    let got = &reply[..(n as usize).min(reply.len())];
    match reply_body(got, OP_PROOF_ROUTE, 1)? {
        (0, body) => read_answer(body),
        _ => None,
    }
}
