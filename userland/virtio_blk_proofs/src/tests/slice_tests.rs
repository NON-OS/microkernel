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

//! Each sleep is the slice or what is left of the budget, whichever is
//! less; a completion is looked for after every sleep, so one whose
//! interrupt was lost is found a slice later; and a clock that cannot be
//! read spends the budget rather than holding the wait open.

use nonos_libc::{break_clock, now, on_wait, waits, Wake};

use super::model::{Rig, IRQ};
use crate::constants::{FLUSH_MS, REQUEST_MS, WAIT_SLICE_MS};
use crate::io::{submit, BlkError};
use crate::queue::{Direction, Fault};

#[test]
fn a_quiet_line_sleeps_out_the_budget_exactly_and_a_flush_has_the_longer_one() {
    for (dir, budget) in [(Direction::Write, REQUEST_MS), (Direction::Flush, FLUSH_MS)] {
        let rig = Rig::modern();
        let mut q = rig.queue();
        let start = now();
        let r = submit(rig.transport(), &mut q, IRQ, dir, 0, 1);
        assert_eq!((r, now() - start), (Err(BlkError::Timeout), budget), "{dir:?}");
    }
}

#[test]
fn a_completion_whose_interrupt_never_came_is_found_after_one_slice() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    let dev = rig.clone();
    on_wait(move |timeout| {
        dev.answer(1);
        Wake { after_ms: timeout, fired: false }
    });
    assert_eq!(submit(rig.transport(), &mut q, IRQ, Direction::Flush, 0, 0), Ok(()));
    assert_eq!(now(), WAIT_SLICE_MS);
}

#[test]
fn a_clock_that_stops_answering_ends_the_wait_at_once() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    break_clock(true);
    let r = submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1);
    assert_eq!((r, waits()), (Err(BlkError::Timeout), 0), "a wait with no clock was begun");
    assert_eq!(q.stalled(), Some(Fault::TimedOut));
}
