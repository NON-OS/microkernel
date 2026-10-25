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

//! A completion that lands while the wait is parked, in the very park after
//! which its deadline has passed, is taken, not reported as a timeout: the
//! controller answered in time, the driver was not running to see it.

use core::cell::Cell;

use super::ring::{cursor_at, entry, ok_status, RINGS};
use crate::admin::{wait_for_completion, Completion, DEADLINE_CHECK_SPINS};

/// The wait for command `cid`, with its completion written during the
/// deadline check that finds the time spent, and `land` saying whether it
/// is written at all.
fn landed_at_deadline(entries: u16, head: u16, cid: u16, land: bool) -> (bool, u64) {
    let written = Cell::new(false);
    let reads = Cell::new(0u64);
    let mut c = cursor_at(entries, head, true);
    let result = wait_for_completion(
        &mut c,
        1,
        cid,
        |at| -> Completion {
            reads.set(reads.get() + 1);
            assert!(reads.get() < 1_000_000, "the wait did not end");
            match at == head && written.get() {
                true => entry(cid, 1, 0, ok_status(true)),
                false => entry(0, 1, 0, ok_status(false)),
            }
        },
        |_| {},
        || {
            written.set(land);
            true
        },
    );
    (result.is_ok(), reads.get())
}

#[test]
fn a_completion_written_while_the_deadline_passes_is_taken() {
    for entries in RINGS {
        for head in 0..entries {
            let (ok, reads) = landed_at_deadline(entries, head, 9, true);
            assert!(ok, "the completion at slot {head} of {entries} was given up on");
            assert_eq!(reads, DEADLINE_CHECK_SPINS as u64 + 1);
        }
    }
}

#[test]
fn with_nothing_written_the_last_look_still_times_out() {
    for entries in RINGS {
        assert_eq!(
            landed_at_deadline(entries, 0, 9, false),
            (false, DEADLINE_CHECK_SPINS as u64 + 1)
        );
    }
}
