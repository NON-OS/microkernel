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

//! A completion that comes late, or never: one that lands after its own
//! wait gave up does not wedge the queue, and every wait ends once its
//! deadline passes.

use super::ring::{cursor_at, entry, ok_status, RINGS};
use super::wait::wait_on;
use crate::admin::{CqCursor, DEADLINE_CHECK_SPINS};
use crate::error::NvmeError;

#[test]
fn a_late_completion_does_not_wedge_the_queue() {
    // Command 7's wait timed out; its completion lands at the head while the
    // driver waits on command 8, whose own completion follows it.
    for entries in RINGS.into_iter().filter(|&n| n >= 2) {
        for head in 0..entries {
            let mut ring = vec![entry(0, 1, 0, ok_status(false)); entries as usize];
            let next = (head + 1) % entries;
            let next_phase = next != 0;
            ring[head as usize] = entry(7, 1, 0, ok_status(true));
            ring[next as usize] = entry(8, 1, 0, ok_status(next_phase));
            let mut c = cursor_at(entries, head, true);
            let w = wait_on(&mut c, 1, 8, &ring, 2);
            assert!(w.result.is_ok(), "command 8 behind a late 7, head {head} of {entries}");
            let after = (next + 1) % entries;
            assert_eq!(w.rung, vec![next, after]);
            assert_eq!(c.head, after);
        }
    }
}

#[test]
fn every_wait_ends_once_the_deadline_passes() {
    for entries in RINGS {
        for allowed in [0u64, 1, 5] {
            let ring = vec![entry(1, 0, 0, ok_status(false)); entries as usize];
            let mut c = CqCursor::new(entries);
            let w = wait_on(&mut c, 0, 1, &ring, allowed);
            assert!(matches!(w.result, Err(NvmeError::ControllerTimeout)));
            assert_eq!(w.checks, allowed + 1);
            // One more read after the deadline: the last look (late_tests).
            assert_eq!(w.reads, (allowed + 1) * DEADLINE_CHECK_SPINS as u64 + 1);
        }
    }
}
