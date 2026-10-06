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

//! A call that takes from a queue for good checks the caller's buffer first.
//!
//! The real handlers are read by path. MkInputEventDrain drained up to 64
//! events from the input ring and only then found the output buffer bad, so
//! the keystrokes were gone and the call said EFAULT; MkProcOutput did the
//! same with a child's line of output. Each check below fails against that
//! code. MkStdinRead and the IPC receives already checked first and are held
//! to it.

const DRAIN: &str = include_str!("../../../src/syscall/dispatch/router/input_ops/do_drain.rs");
const PROC_OUTPUT: &str = include_str!("../../../src/syscall/microkernel/proc_output.rs");
const STDIN_READ: &str = include_str!("../../../src/syscall/microkernel/proc_stdin/read.rs");
const RECV: &str = include_str!("../../../src/syscall/microkernel/ipc/recv.rs");
const RECV_FROM: &str = include_str!("../../../src/syscall/microkernel/ipc/recv_from.rs");

/// Byte offset of `needle` in `text`, which must hold it.
fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

/// The body of the function `name` in `src`, up to its closing brace.
fn body<'a>(src: &'a str, name: &str) -> &'a str {
    let from = &src[at(src, name)..];
    &from[..at(from, "\n}\n")]
}

/// In `f`, the buffer check comes before the take, and both are there.
fn checks_before_taking(f: &str, check: &str, take: &str) -> bool {
    match (f.find(check), f.find(take)) {
        (Some(c), Some(t)) => c < t,
        _ => false,
    }
}

#[test]
fn the_input_drain_checks_the_buffer_before_draining_the_ring() {
    let f = body(DRAIN, "pub(super) fn do_drain");
    let check = "validate_user_write(out_ptr, cap * core::mem::size_of::<InputEvent>())";
    assert!(checks_before_taking(f, check, "drain_input("));
}

#[test]
fn proc_output_checks_the_buffer_before_taking_the_line() {
    let f = body(PROC_OUTPUT, "pub fn sys_proc_output");
    let check = "crate::usercopy::validate_user_write(buf_ptr, buf_len)";
    assert!(checks_before_taking(f, check, "try_dequeue_existing("));
}

#[test]
fn the_other_takers_still_check_first() {
    let stdin = body(STDIN_READ, "pub fn sys_stdin_read");
    assert!(checks_before_taking(stdin, "validate_user_write(buf_ptr, buf_len)", "take_front("));
    let recv = body(RECV, "pub fn sys_ipc_recv");
    assert!(checks_before_taking(recv, "validate_user_write(buf, len)", "recv_from_inbox("));
    let from = body(RECV_FROM, "pub fn sys_ipc_recv_from");
    assert!(checks_before_taking(from, "validate_user_write(buf, len)", "drain("));
}

#[test]
fn the_order_check_sees_a_take_before_a_check() {
    let wrong = "let n = drain_input(&mut s);\nvalidate_user_write(out_ptr, 8);";
    assert!(!checks_before_taking(wrong, "validate_user_write(", "drain_input("));
    assert!(!checks_before_taking("drain_input(&mut s);", "validate_user_write(", "drain_input("));
}
