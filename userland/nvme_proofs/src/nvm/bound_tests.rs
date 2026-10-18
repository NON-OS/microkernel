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

//! A request bounded by a recovery (answer_by): no command goes out once its
//! answer is due, and it is answered within its own 33 s however late.

use super::constants::REPLY_BY_MS;
use super::model::{io_queue, post};
use super::{sq_tail_doorbell, IoQueue};
use crate::completion_tests::entry;
use crate::error::NvmeError;
use crate::fake::{register, registers};

/// Internal Error (SCT 0, SC 06h), phase 1: transient, worth a retry.
const INTERNAL: u16 = (0x06 << 1) | 1;

/// Read one block due `left` ms after it starts, command 1 failing, command
/// 2 completing; the clock moves 1 s a read, so the retry check reads 2 s.
fn read_bounded(left: u64) -> (Result<(), NvmeError>, u32) {
    nonos_libc::set_clock(1_000);
    let (regs, mut io) = (registers(), io_queue());
    post(&io, 0, entry(1, 1, 0, INTERNAL));
    post(&io, 1, entry(2, 1, 0, 1));
    // This read is at `now`; the request starts at the next, 1 s on.
    let now = nonos_libc::mk_uptime_ms() as u64;
    io.answer_by = Some(now + 1_000 + left);
    let r = io.retried(0, |io| io.transfer(regs, 0, 1, false));
    (r, register(regs, sq_tail_doorbell(1, 0)))
}

#[test]
fn a_request_already_due_sends_nothing() {
    let (r, sent) = read_bounded(0);
    assert!(matches!(r, Err(NvmeError::ControllerTimeout)), "{r:?}");
    assert_eq!(sent, 0, "a command went out at its answer");
}

#[test]
fn a_retry_goes_out_only_while_the_request_has_time() {
    // Due at 2 s, as the retry check reads: no retry; at 2.1 s, it goes.
    let (r, sent) = read_bounded(2_000);
    assert_eq!((matches!(r, Err(NvmeError::AdminCommandFailed)), sent), (true, 1));
    let (r, sent) = read_bounded(2_100);
    assert_eq!((r.is_ok(), sent), (true, 2));
}

#[test]
fn a_bound_passed_on_past_33_s_still_leaves_the_reply_its_2_s() {
    // Passed on later than its own 33 s; held up 5 s, then stalled.
    nonos_libc::set_clock(100);
    let (regs, mut io) = (registers(), io_queue());
    io.answer_by = Some(nonos_libc::mk_uptime_ms() as u64 + 100);
    assert!(io.retried(0, |_| Ok(())).is_err(), "refused at its bound");
    let start = nonos_libc::mk_uptime_ms() as u64 + 100;
    let r = io.retried(0, |io: &mut IoQueue| {
        (0..50).for_each(|_| _ = nonos_libc::mk_uptime_ms());
        io.transfer(regs, 0, 1, false)
    });
    let after = nonos_libc::mk_uptime_ms() as u64 - start;
    let timed_out = matches!(r, Err(NvmeError::ControllerTimeout));
    assert!(timed_out && after <= REPLY_BY_MS + 200, "{r:?} {after} ms after it was taken");
}
