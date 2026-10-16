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

//! A write whose data phase fails still waits for the card to program what
//! it took (card/finish.rs), one block or many, and answers the data
//! phase's own error inside the kernel's wait. A stalled single-block
//! write runs the data phase's limit and then the programming wait, inside
//! its sum (worst_case_tests.rs) and below the multi-block write's.

use nonos_libc::now_ms;

use super::rig::{knobs, sdhc, serving};
use super::service_rig::{call, span, KERNEL};
use super::worst_case_tests::{KERNEL_WAIT_MS, SINGLE_WRITE};
use crate::card::limits::{DATA_MS, PROGRAM_MS};
use crate::model::{Card, Knobs};
use crate::protocol::*;

/// `count` blocks written to `card` with `k` turned: the status, and the ms.
fn timed_write(card: Card, k: Knobs, count: u32) -> (i32, u64) {
    let (mut drv, mut medium) = serving(card);
    knobs(k);
    let t0 = now_ms();
    let body = [span(0, count), vec![1; count as usize * 512]].concat();
    (call(&mut drv, &mut medium, KERNEL, OP_WRITE, &body).status, now_ms() - t0)
}

/// A failed write to a card that never finishes programming it.
fn failed_to_a_slow_card(count: u32) -> (i32, u64) {
    let mut slow = sdhc(1);
    slow.program_polls = u32::MAX;
    timed_write(slow, Knobs { data_error: true, ..Knobs::default() }, count)
}

#[test]
fn a_failed_write_whose_card_never_finishes_programming_is_answered_in_time() {
    let (status, ms) = failed_to_a_slow_card(128);
    assert_eq!(status, E_IO, "the data phase's error, not the wait's");
    assert!((PROGRAM_MS..KERNEL_WAIT_MS).contains(&ms), "{ms} ms");
}

#[test]
fn a_failed_single_block_write_waits_out_programming_too() {
    let (status, ms) = failed_to_a_slow_card(1);
    assert_eq!(status, E_IO, "the data phase's error, not the wait's");
    assert!((PROGRAM_MS..KERNEL_WAIT_MS).contains(&ms), "{ms} ms");
}

#[test]
fn a_stalled_single_block_write_is_answered_inside_its_sum() {
    let (status, ms) = timed_write(sdhc(1), Knobs { stall_data: true, ..Knobs::default() }, 1);
    assert_eq!(status, E_TIMEDOUT);
    assert!((DATA_MS + PROGRAM_MS..SINGLE_WRITE).contains(&ms), "{ms} ms");
}
