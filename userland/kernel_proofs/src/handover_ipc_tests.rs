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

//! The IPC receives take a message for good, so each checks what it writes
//! to before it waits: the buffer, and MkIpcRecvReq's meta with it. The
//! real handlers are read by path (handover_tests.rs).

use crate::proofs::proofs_3::handover_tests::{body, checks_before_taking};

const RECV: &str = include_str!("../../../src/syscall/microkernel/ipc/recv.rs");
const RECV_FROM: &str = include_str!("../../../src/syscall/microkernel/ipc/recv_from.rs");
const RECV_REQ: &str = include_str!("../../../src/syscall/microkernel/ipc/recv_req.rs");
const RECV_OPEN: &str = include_str!("../../../src/syscall/microkernel/ipc/recv_open.rs");

#[test]
fn the_ipc_receives_check_the_buffer_before_taking() {
    let recv = body(RECV, "pub fn sys_ipc_recv");
    assert!(checks_before_taking(recv, "validate_user_write(buf, len)", "recv_from_inbox("));
    let buffer = body(RECV_OPEN, "pub(super) fn buffer");
    assert!(buffer.contains("validate_user_write(buf, len)"));
    let from = body(RECV_FROM, "pub fn sys_ipc_recv_from");
    assert!(checks_before_taking(from, "buffer(buf, len)", "drain("));
    let req = body(RECV_REQ, "pub fn sys_ipc_recv_req");
    assert!(checks_before_taking(req, "buffer(buf, len)", "drain("));
}

#[test]
fn mk_ipc_recv_req_checks_its_meta_before_taking() {
    let req = body(RECV_REQ, "pub fn sys_ipc_recv_req");
    assert!(checks_before_taking(req, "meta_len_ok(meta_len)", "drain("));
    assert!(checks_before_taking(req, "validate_user_write(meta_ptr, META_V1_LEN)", "drain("));
}
