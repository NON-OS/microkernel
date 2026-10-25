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


//! The learned receive bound: a fast driver keeps its floor, a slow one is
//! waited for up to the ceiling, and a run of timeouts raises the bound.

use core::cell::Cell;
use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::{last_timeout_ms, serial, set_responder};

use crate::device::batch_call::call;
use crate::device::learned_bound::{bound, learn, sample, BATCH, SINGLE};
use crate::device::rx_learned::{timed_with, BATCH_MS};
use crate::device::rx_summary::due;

#[test]
fn a_fresh_op_starts_at_its_floor_and_never_passes_its_ceiling() {
    assert_eq!(bound(&BATCH, 0), 24);
    assert_eq!(bound(&SINGLE, 0), 8);
    assert_eq!(bound(&BATCH, 14), 28);
    assert_eq!(bound(&BATCH, 100), 32);
    assert_eq!(bound(&SINGLE, 100), 24);
}

#[test]
fn a_timeout_teaches_at_least_the_wait() {
    assert_eq!(sample(false, 24, 24), 24);
    assert_eq!(sample(false, 30, 24), 30);
    assert_eq!(sample(true, 3, 24), 3);
    assert_eq!(learn(0, 8), 1);
    assert_eq!(learn(8, 8), 8);
}

/// A clock moved by the calls themselves: each takes `cost` ms.
fn run(slot: &AtomicU64, op: &crate::device::learned_bound::OpBound, cost: u64, answers: bool) -> u64 {
    let t = Cell::new(0i64);
    let mut waited = 0;
    timed_with(slot, op, || t.get(), |ms| {
        waited = ms;
        t.set(cost.min(ms) as i64);
        if answers && cost <= ms { 0 } else { -110 }
    });
    waited
}

#[test]
fn a_slow_driver_is_waited_for_up_to_the_ceiling() {
    let slot = AtomicU64::new(0);
    let first = run(&slot, &BATCH, 60, false);
    assert_eq!(first, 24);
    let mut last = first;
    for _ in 0..40 {
        last = run(&slot, &BATCH, 60, false);
    }
    assert_eq!(last, 32, "timeouts must raise the bound to the ceiling");
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
