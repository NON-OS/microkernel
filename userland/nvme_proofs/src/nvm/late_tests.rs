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

//! A command whose wait ran out stays the controller's until its completion
//! is seen, and that completion retires it whatever its status: the error
//! belongs to the request already answered, and the next one goes ahead.

use super::model::{io_queue, post};
use super::{sq_tail_doorbell, IoQueue};
use crate::completion_tests::entry;
use crate::error::NvmeError;
use crate::fake::{register, registers};
use crate::protocol::device_status;
use crate::regs::Regs;

/// Unrecovered Read Error: media and data integrity (SCT 2), SC 81h, phase 1.
const UNRECOVERED_READ: u16 = (2 << 9) | (0x81 << 1) | 1;

/// A queue whose first read, command 1, got no completion in its 30 s.
fn given_up() -> (Regs, IoQueue) {
    nonos_libc::set_clock(1_000);
    let (regs, mut io) = (registers(), io_queue());
    let first = io.transfer(regs, 0, 1, false);
    assert!(matches!(first, Err(NvmeError::ControllerTimeout)), "{first:?}");
    assert_eq!(io.out, Some(1), "command 1 is still the controller's");
    (regs, io)
}

#[test]
fn a_late_completion_with_an_error_retires_its_command() {
    for (late, what) in [(UNRECOVERED_READ, "an error"), (0xffff, "every bit"), (1, "success")] {
        let (regs, mut io) = given_up();
        post(&io, 0, entry(1, 1, 0, late)); // late, and command 2 at once after it
        post(&io, 1, entry(2, 1, 0, 1));
        let second = io.transfer(regs, 0, 1, false);
        assert!(second.is_ok(), "the disk stays usable after {what}: {second:?}");
        assert_eq!(io.out, None, "nothing is left out after {what}");
        post(&io, 2, entry(3, 1, 0, 1));
        assert!(io.transfer(regs, 0, 1, true).is_ok(), "and so does the one after");
    }
}

#[test]
fn the_next_request_answers_with_its_own_status_not_the_late_one() {
    let (regs, mut io) = given_up();
    post(&io, 0, entry(1, 1, 0, UNRECOVERED_READ));
    // Command 2's own failure: Invalid Field in Command (SCT 0, SC 02h).
    post(&io, 1, entry(2, 1, 0, (0x02 << 1) | 1));
    let Err(e) = io.transfer(regs, 0, 1, false) else { panic!("command 2 failed") };
    assert_eq!(io.failure_status(e), device_status(0, 0x02));
}

#[test]
fn a_command_never_seen_holds_the_buffer_and_the_queue() {
    let (regs, mut io) = given_up();
    // Neither silence nor command 1's id on another queue settles it.
    post(&io, 0, entry(1, 2, 0, 1));
    let again = io.transfer(regs, 0, 1, false);
    assert!(matches!(again, Err(NvmeError::ControllerTimeout)), "{again:?}");
    assert_eq!(io.out, Some(1));
    assert_eq!(register(regs, sq_tail_doorbell(1, 0)), 1, "nothing more was submitted");
}
