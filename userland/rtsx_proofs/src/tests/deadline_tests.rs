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

//! Every request answers inside the 35 s the kernel waits for a reply
//! (src/services/lifecycle/reply_wait.rs), the driver naming why: the
//! slowest the model can make, end to end with the slot looked at first.
//! The longest by its parts is in worst_case_tests.rs, and a failed write
//! waited out in write_wait_tests.rs.

use nonos_libc::now_ms;

use super::rig::{knobs, sdhc, serving};
use super::service_rig::{call, span, KERNEL};
use super::worst_case_tests::KERNEL_WAIT_MS;
use crate::card::limits::PROGRAM_MS;
use crate::model::{with, Knobs, SD_INT};
use crate::protocol::*;

#[test]
fn a_card_that_never_finishes_programming_is_answered_in_time() {
    let mut slow = sdhc(1);
    slow.program_polls = u32::MAX;
    let (mut drv, mut medium) = serving(slow);
    let t0 = now_ms();
    let w = call(&mut drv, &mut medium, KERNEL, OP_WRITE, &[span(0, 128), vec![1; 65536]].concat());
    assert_eq!(w.status, E_TIMEDOUT);
    assert!((PROGRAM_MS..KERNEL_WAIT_MS).contains(&(now_ms() - t0)), "{} ms", now_ms() - t0);
}

#[test]
fn a_swap_seen_first_and_a_mute_card_still_answer_in_time() {
    let (mut drv, mut medium) = serving(sdhc(1));
    knobs(Knobs { stall_data: true, ..Knobs::default() });
    let t0 = now_ms();
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 128)).status, E_TIMEDOUT);
    assert!(now_ms() - t0 < KERNEL_WAIT_MS, "{} ms", now_ms() - t0);
    knobs(Knobs::default());
    let mut mute = sdhc(1);
    mute.busy_polls = u32::MAX;
    with(|r| (r.pull().map(|_| r.insert(mute)), r.pending |= SD_INT));
    let t1 = now_ms();
    assert_eq!(call(&mut drv, &mut medium, KERNEL, OP_READ, &span(0, 1)).status, E_NODEV);
    assert!(now_ms() - t1 < KERNEL_WAIT_MS, "{} ms", now_ms() - t1);
}
