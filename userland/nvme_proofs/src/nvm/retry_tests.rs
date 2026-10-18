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

//! A failed I/O command goes again only while its completion allows: Do Not
//! Retry clear and no media error, at most RETRIES times. A command that
//! never completed is never sent again from here.

use super::model::{io_queue, post};
use super::retry::{worth_retrying, RETRIES};
use super::sq_tail_doorbell;
use crate::completion_tests::entry;
use crate::error::NvmeError;
use crate::fake::{register, registers};
use crate::protocol::device_status;

/// Internal Error (SCT 0, SC 06h) and Unrecovered Read Error (SCT 2, SC
/// 81h), each with phase 1; Do Not Retry is bit 15.
const INTERNAL: u16 = (0x06 << 1) | 1;
const MEDIA: u16 = (2 << 9) | (0x81 << 1) | 1;
const DNR: u16 = 0x8000;

/// Read one block with `statuses` posted for commands 1 and on: the result,
/// its reply status, and the commands sent.
fn read_with(statuses: &[u16]) -> (Result<(), NvmeError>, i32, u32) {
    nonos_libc::set_clock(100);
    let (regs, mut io) = (registers(), io_queue());
    for (i, &s) in statuses.iter().enumerate() {
        post(&io, i as u16, entry(i as u16 + 1, 1, 0, s));
    }
    let r = io.retried(0, |io| io.transfer(regs, 0, 1, false));
    let status = r.map_or_else(|e| io.failure_status(e), |()| 0);
    (r, status, register(regs, sq_tail_doorbell(1, 0)))
}

#[test]
fn a_transient_failure_goes_again_and_succeeds() {
    let (r, status, sent) = read_with(&[INTERNAL, 1]);
    assert_eq!((r.is_ok(), status, sent), (true, 0, 2));
}

#[test]
fn do_not_retry_and_media_errors_are_answered_at_once() {
    for (s, sct, sc) in [(INTERNAL | DNR, 0, 0x06), (MEDIA, 2, 0x81), (MEDIA | DNR, 2, 0x81)] {
        let (r, status, sent) = read_with(&[s, 1]);
        assert_eq!((r.is_err(), status, sent), (true, device_status(sct, sc), 1), "{s:#x}");
    }
}

#[test]
fn retries_are_capped_and_a_lost_command_is_never_resent() {
    let (r, status, sent) = read_with(&[INTERNAL; 8]);
    assert_eq!((r.is_err(), status, sent), (true, device_status(0, 0x06), 1 + RETRIES));
    let (r, _, sent) = read_with(&[]);
    assert!(matches!(r, Err(NvmeError::ControllerTimeout)), "{r:?}");
    assert_eq!(sent, 1, "a command that may still be out goes to recovery");
}

#[test]
fn every_status_word_is_judged_by_dnr_and_its_type_alone() {
    let rule = |s: u16| s & DNR == 0 && (s >> 9) & 7 != 2;
    assert!((0..=u16::MAX).all(|s| worth_retrying(s) == rule(s)));
}
