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

//! What to do with the result of one fetch step.

use core::sync::atomic::Ordering;

use super::directory_clock::{now_ms, BACKOFF_MS, FIRST_BACKOFF_MS, MAX_BACKOFF_MS, NEXT_TRY_MS};
use crate::directory_sync::Step;
use crate::trace;

/// How long to wait before re-syncing when a directory arrived with gateways
/// but no exit.
const RESYNC_FOR_EXIT_MS: u64 = 250;

/// Record a step, and hold off after one that did not arrive.
pub fn record(step: Step) {
    match step {
        Step::Progressed => {
            trace::say(b"directory: mix layers in hand, gateways next");
            BACKOFF_MS.store(FIRST_BACKOFF_MS, Ordering::Relaxed);
        }
        Step::Done(count) => {
            trace::say_num(b"directory synced, nodes", count as u64);
            // The gateway in hand was dialled before there was a directory,
            // so it is probably not in the one that just arrived.
            super::rebind::rebind_if_unknown();
            BACKOFF_MS.store(FIRST_BACKOFF_MS, Ordering::Relaxed);
            // A sync missing a gateway or an exit is not usable: the tick will
            // run again because one of those counts is still zero, but
            // re-fetching the whole list every tick would hammer the
            // directory.
            if crate::state::directory_gateway_count() == 0
                || crate::state::directory_exit_count() == 0
            {
                hold(RESYNC_FOR_EXIT_MS);
            }
        }
        Step::Failed(code) => {
            trace::say_num(b"directory sync failed, code", code as u64);
            let wait = BACKOFF_MS.load(Ordering::Relaxed);
            hold(wait);
            BACKOFF_MS.store((wait * 2).min(MAX_BACKOFF_MS), Ordering::Relaxed);
        }
    }
}

/// Refuse the next attempt until `wait` milliseconds from now.
fn hold(wait: u64) {
    NEXT_TRY_MS.store(now_ms().saturating_add(wait), Ordering::Relaxed);
}
