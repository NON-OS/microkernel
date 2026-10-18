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

//! The interrupt wait, over a delivery count the fake controller moves when
//! it posts an I/O completion (`raise`). A wait returns at once either way:
//! the test's controller acts on clock reads, and the driver reads the clock
//! straight after every park. As the kernel answers, a count that moved past
//! the one the driver last saw is 0 and the new count, and one that did not
//! is MK_IRQ_WAIT_TIMED_OUT, a wait that slept out its timeout. Each wait is
//! recorded with its grant and its timeout, which must be 1 or 2 ms: 0 asks
//! for the kernel's default, and anything longer stalls a completion that
//! raised no interrupt.

use std::cell::Cell;

use crate::calls::record;

/// What the kernel returns for a wait whose whole timeout passed with no
/// delivery (nonos_libc's broker::irq_wait).
pub const MK_IRQ_WAIT_TIMED_OUT: i64 = 1;
/// A grant the kernel refuses, so a test can see the driver drop it.
pub const REFUSED_GRANT: u64 = 0xdead;

thread_local! {
    static DELIVERED: Cell<u64> = const { Cell::new(0) };
}

/// Deliver the interrupt, as the controller does when it posts a completion
/// on a queue created with interrupts enabled.
pub fn raise() {
    DELIVERED.with(|d| d.set(d.get() + 1));
}

pub fn mk_irq_wait(grant_id: u64, last_seq: u64, timeout_ms: u64, out_seq: &mut u64) -> i64 {
    record("irq_wait", grant_id);
    record("irq_wait_ms", timeout_ms);
    assert!((1..=2).contains(&timeout_ms), "a park of {timeout_ms} ms");
    if grant_id == REFUSED_GRANT {
        return -1;
    }
    let seq = DELIVERED.with(Cell::get);
    *out_seq = seq;
    if seq == last_seq {
        MK_IRQ_WAIT_TIMED_OUT
    } else {
        0
    }
}

pub fn mk_yield() -> i64 {
    0
}
