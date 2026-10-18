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

//! The device's interrupt line as MkIrqWait, MkIrqPoll and MkIrqAck answer
//! it. Each wait takes the time the script says, never past its own
//! timeout, and the clock moves by that much. A script may change the ring
//! as it answers, which is how a device completes a request while the
//! driver sleeps.

use std::cell::Cell;

use crate::clock::advance;
use crate::line::{answer, ACKS, SEQ, WAITS};
use crate::types::irq::IrqPollOut;

/// The wait slept out its whole timeout with the sequence unmoved.
pub const MK_IRQ_WAIT_TIMED_OUT: i64 = 1;

pub fn mk_irq_wait(_grant: u64, last_seq: u64, timeout_ms: u64, out_seq: *mut u64) -> i64 {
    WAITS.with(|w| w.set(w.get() + 1));
    let wake = answer(timeout_ms);
    advance(wake.after_ms.min(timeout_ms));
    if wake.fired {
        SEQ.with(|s| s.set(s.get() + 1));
    }
    let seq = SEQ.with(Cell::get);
    // SAFETY: the driver passes the address of one of its own u64s.
    unsafe { *out_seq = seq };
    let slept_out = !wake.fired && wake.after_ms >= timeout_ms && seq == last_seq;
    if slept_out {
        MK_IRQ_WAIT_TIMED_OUT
    } else {
        0
    }
}

pub fn mk_irq_poll(_grant: u64, out: *mut IrqPollOut) -> i64 {
    // SAFETY: the driver passes the address of one of its own records.
    unsafe { (*out).seq = SEQ.with(Cell::get) };
    0
}

pub fn mk_irq_ack(_grant: u64) -> i64 {
    ACKS.with(|a| a.set(a.get() + 1));
    0
}
