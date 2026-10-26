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

//! The learned bound in use: a fast driver keeps its floor, timeouts raise a
//! slow one's to the ceiling, the batch call waits it, the summary is sparse.

use core::cell::Cell;
use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::{last_timeout_ms, serial, set_responder};

use crate::device::batch_call::call;
use crate::device::learned_bound::{OpBound, BATCH, SINGLE};
use crate::device::rx_learned::{timed_with, BATCH_MS};
use crate::device::rx_summary::due;

/// One call to a driver that takes `cost` ms and answers if `answers`; the
/// clock is moved by the call itself. The bound the call was given.
fn run(slot: &AtomicU64, op: &OpBound, cost: u64, answers: bool) -> u64 {
    let (t, waited) = (Cell::new(0i64), Cell::new(0));
    let call = |ms: u64| {
        waited.set(ms);
        t.set(cost.min(ms) as i64);
        -110 * i64::from(!(answers && cost <= ms))
    };
    timed_with(slot, op, || t.get(), call);
    waited.get()
}

#[test]
fn a_slow_driver_is_waited_for_up_to_the_ceiling() {
    let slot = AtomicU64::new(0);
    assert_eq!(run(&slot, &BATCH, 60, false), 24);
    let last = (0..40).map(|_| run(&slot, &BATCH, 60, false)).last();
    assert_eq!(last, Some(32), "timeouts must raise the bound to the ceiling");
}

#[test]
fn a_fast_driver_keeps_its_floor() {
    let slot = AtomicU64::new(0);
    for _ in 0..40 {
        assert_eq!(run(&slot, &SINGLE, 1, true), 8);
    }
    assert!(slot.load(Ordering::Relaxed) <= 1);
}

#[test]
fn the_batch_call_waits_the_learned_bound() {
    let _g = serial();
    set_responder(|_, _| Vec::new());
    BATCH_MS.store(14, Ordering::Relaxed);
    let _ = call(7, Some(1));
    assert_eq!(last_timeout_ms(), 28);
}

#[test]
fn the_summary_is_said_four_times_at_most() {
    let said: Vec<u64> = (1..=20_000).filter(|&n| due(n)).collect();
    assert_eq!(said, [256, 1024, 4096, 16384]);
}
