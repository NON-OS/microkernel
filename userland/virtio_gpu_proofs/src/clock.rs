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

//! The clock the driver's waits are measured by, as the tests run it: each
//! question a wait asks counts as a millisecond. The time since boot is the
//! milliseconds this thread's waits have counted so far, and any a test lets
//! pass, as it does while a client presents or sits idle.
//!
//! While a part runs on this thread's windows (`parts`), a wait begins only
//! between two of its answers, so a reset the driver makes next never lands
//! in the middle of one; a question asked before the part had a turn since
//! the last gives the processor up, as the shipping clock does at every one;
//! and a wait whose questions are spent ends only once each part has taken a
//! turn since the last question, so a part the scheduler held back is looked
//! at once more rather than read as one that never answered. HELD_BACK of
//! wall time bounds that. Giving the processor up at every question made a
//! stall of 2000 questions 2000 trips through the scheduler, and the suite
//! ten times as long on a loaded host; the turn rule is what keeps a part
//! held back from reading as a timeout.

use std::cell::Cell;
use std::thread::yield_now;
use std::time::{Duration, Instant};

use crate::parts;

const HELD_BACK: Duration = Duration::from_secs(10);

thread_local!(static NOW: Cell<u64> = const { Cell::new(0) });

pub fn expired_after(ms: u64) -> impl FnMut() -> bool {
    parts::between_steps();
    let (mut spent, mut seen, mut since) = (0u64, parts::turns(), None);
    move || {
        spent += 1;
        pass(1);
        let now = parts::turns();
        let turned = now.len() != seen.len() || now.iter().zip(&seen).all(|(n, s)| n != s);
        seen = now;
        if !turned {
            yield_now();
        }
        spent >= ms && (turned || since.get_or_insert_with(Instant::now).elapsed() >= HELD_BACK)
    }
}

/// Milliseconds since boot.
pub fn now_ms() -> u64 {
    NOW.with(Cell::get)
}

/// Let `ms` go by.
pub fn pass(ms: u64) {
    NOW.with(|now| now.set(now.get().saturating_add(ms)));
}
