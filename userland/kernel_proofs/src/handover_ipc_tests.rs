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

//! The IPC receives take a message for good, so each checks the buffer it
//! writes to before it waits. The real handlers are read by path
//! (handover_tests.rs).

use crate::proofs::proofs_3::handover_tests::{body, checks_before_taking};

const RECV: &str = include_str!("../../../src/syscall/microkernel/ipc/recv.rs");
const RECV_FROM: &str = include_str!("../../../src/syscall/microkernel/ipc/recv_from.rs");

#[test]
fn the_ipc_receives_check_the_buffer_before_taking() {
    let recv = body(RECV, "pub fn sys_ipc_recv");
    assert!(checks_before_taking(recv, "validate_user_write(buf, len)", "recv_from_inbox("));
    let from = body(RECV_FROM, "pub fn sys_ipc_recv_from");
    assert!(checks_before_taking(from, "validate_user_write(buf, len)", "drain("));
}
