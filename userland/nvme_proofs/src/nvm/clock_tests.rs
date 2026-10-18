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

//! With no clock no wait can be bounded, so no command goes out: a request
//! taken while the clock cannot be read is refused, and the bound a recovery
//! left is the kernel's request's still (and a bound set without the clock,
//! no_clock_tests).

use super::model::{io_queue, post};
use super::sq_tail_doorbell;
use crate::completion_tests::entry;
use crate::error::NvmeError;
use crate::fake::{register, registers};

#[test]
fn a_request_taken_without_a_clock_sends_nothing() {
    let (regs, mut io) = (registers(), io_queue());
    post(&io, 0, entry(1, 1, 0, 1));
    nonos_libc::fail_clock(true);
    let r = io.retried(0, |io| io.transfer(regs, 0, 1, false));
    nonos_libc::fail_clock(false);
    assert!(matches!(r, Err(NvmeError::ClockFailed)), "{r:?}");
    assert_eq!(register(regs, sq_tail_doorbell(1, 0)), 0, "a command went out");
}

#[test]
fn a_request_refused_without_a_clock_leaves_the_kernels_bound() {
    // Another sender's leaves the bound for the kernel's request; the
    // kernel's passes it on, as every refusal of its request does.
    let (regs, mut io) = (registers(), io_queue());
    io.bound_next(0);
    let bound = io.answer_by.expect("a bound");
    nonos_libc::fail_clock(true);
    let theirs = io.retried(42, |io| io.transfer(regs, 0, 1, false));
    let left = io.answer_by;
    let kernels = io.retried(0, |io| io.transfer(regs, 0, 1, false));
    nonos_libc::fail_clock(false);
    let refused = [theirs, kernels].map(|r| matches!(r, Err(NvmeError::ClockFailed)));
    assert_eq!(refused, [true, true]);
    assert_eq!((left, io.answer_by), (Some(bound), Some(bound + 35_000)));
}
