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

//! A serve loop's turn after its receive: serve a message, go round after a
//! wait that saw nothing, and sleep before going round after a receive that
//! failed at once, so a loop whose inbox is gone never holds a core.

use crate::bringup_policy::{recv_turn, RecvTurn, ERRNO_TIMEDOUT, RECV_PARK_MS};

#[test]
fn a_message_is_served() {
    assert_eq!(recv_turn(1), RecvTurn::Serve);
    assert_eq!(recv_turn(4096), RecvTurn::Serve);
}

#[test]
fn a_wait_that_saw_nothing_goes_round_without_sleeping() {
    assert_eq!(recv_turn(ERRNO_TIMEDOUT), RecvTurn::Again);
    // An empty message was dequeued; the next receive blocks again.
    assert_eq!(recv_turn(0), RecvTurn::Again);
}

#[test]
fn a_receive_that_failed_at_once_sleeps_before_the_next() {
    // ENOENT (no inbox), EFAULT, EINVAL, EPERM: each would fail again at once.
    for rc in [-2, -14, -22, -1, i64::MIN] {
        assert_eq!(recv_turn(rc), RecvTurn::Park, "rc {rc}");
    }
    const { assert!(RECV_PARK_MS >= 10, "a park short enough to be a spin") };
    const { assert!(RECV_PARK_MS <= 1_000, "a park long enough to stall a recovered loop") };
}

#[test]
fn the_timeout_is_the_kernels() {
    let errnos = include_str!("../../../src/syscall/microkernel/errnos.rs");
    assert!(errnos.contains("pub const ERRNO_TIMEDOUT: i64 = -110;"));
}
