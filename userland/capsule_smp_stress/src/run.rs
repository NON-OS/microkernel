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

//! One run: start the workers, report every minute, stop them at the end and
//! give the verdict. The time printed is the time measured, never rounded.

use crate::clock::now_ms;
use crate::stats::{self, STOP};
use crate::{report, stall, workers};
use core::sync::atomic::Ordering;
use nonos_libc::mk_idle_ms;

const REPORT_EVERY_MS: u64 = 60_000;
/// Every wait a worker makes ends within its timeout, so this is ample for
/// all of them to see the stop and end.
const WIND_DOWN_MS: u64 = 3 * stall::WAIT_TIMEOUT_MS;

/// The exit status: 0 on a pass, 1 otherwise.
pub fn run(seconds: u64) -> i32 {
    let start = now_ms();
    report::started(seconds, workers::TOTAL);
    if let Err((seat, errno)) = workers::spawn_all() {
        report::refused(seat, errno);
        STOP.store(true, Ordering::Release);
        wind_down();
        return 1;
    }
    let end = start.saturating_add(seconds.saturating_mul(1000));
    let mut next_report = start.saturating_add(REPORT_EVERY_MS);
    loop {
        mk_idle_ms(1000);
        let now = now_ms();
        if now >= end {
            break;
        }
        if now >= next_report {
            report::progress(now - start, &stats::snapshot());
            next_report = next_report.saturating_add(REPORT_EVERY_MS);
        }
    }
    let ran_ms = now_ms().saturating_sub(start);
    STOP.store(true, Ordering::Release);
    let left = wind_down();
    let counts = stats::snapshot();
    let pass = stall::verdict(&counts) && left == 0;
    report::finished(ran_ms, &counts, pass, left);
    if pass {
        0
    } else {
        1
    }
}

/// Wait for the workers to end. How many had not when the time ran out.
fn wind_down() -> usize {
    let from = now_ms();
    while workers::running() > 0 && now_ms().saturating_sub(from) < WIND_DOWN_MS {
        mk_idle_ms(10);
    }
    workers::running()
}
