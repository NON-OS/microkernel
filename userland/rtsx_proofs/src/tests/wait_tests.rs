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

//! C7: every wait is by elapsed time, parked between polls once it has
//! gone on past a millisecond, and ends in a named failure. A reader that
//! reads all ones is let go at once, with both engines told to stop.

use nonos_libc::{now_ms, sleeps, uptime_reads};

use crate::clock::{now, pause, Budget};

use super::rig::{knobs, up};
use crate::engine::wait;
use crate::error::RtsxError::{self, CmdFailed, CmdTimeout, DataFailed, DataTimeout};
use crate::hw::read_register;
use crate::model::{with, Knobs};
use crate::regs::pm::DMACTL;

#[test]
fn a_ten_second_wait_is_slept_through_not_spun() {
    let drv = up(None);
    let (t0, polls0, naps0) = (now_ms(), uptime_reads(), sleeps());
    assert_eq!(wait(&drv, 10_000, DataTimeout, DataFailed), Err(DataTimeout));
    let (spent, polls, naps) = (now_ms() - t0, uptime_reads() - polls0, sleeps() - naps0);
    assert!((10_000..10_050).contains(&spent), "{spent} ms");
    assert!(naps >= 9_000 && polls < 20_000, "{polls} clock reads, {naps} sleeps");
}

#[test]
fn an_all_ones_bipr_stops_both_engines_and_is_named() {
    let drv = up(None);
    knobs(Knobs { blink: 1, ..Knobs::default() });
    assert_eq!(wait(&drv, 100, CmdTimeout, CmdFailed), Err(RtsxError::Gone));
    with(|r| assert!(r.cmd_stopped && r.dma_stopped, "engines left armed"));
}

#[test]
fn a_reader_gone_from_the_bus_fails_a_register_access_at_once() {
    let drv = up(None);
    knobs(Knobs { gone: true, ..Knobs::default() });
    let t0 = now_ms();
    assert_eq!(read_register(drv.regs, DMACTL), Err(RtsxError::Gone));
    assert!(now_ms() - t0 < 2, "{} ms", now_ms() - t0);
    knobs(Knobs::default());
}

#[test]
fn a_wait_begun_late_in_a_millisecond_still_spins_a_whole_one() {
    let tick = now();
    while now() == tick {}
    // Each read is 2 us: the budget starts 2 us before the next tick.
    for _ in 0..498 {
        let _ = now();
    }
    let (budget, reads, naps) = (Budget::begin(now(), 100), uptime_reads(), sleeps());
    while sleeps() == naps && uptime_reads() - reads < 100_000 {
        pause(&budget, now());
    }
    assert!(sleeps() > naps, "never slept");
    let spun_us = (uptime_reads() - reads) * 2;
    assert!(spun_us >= 1_000, "spun {spun_us} us before the first sleep");
}
