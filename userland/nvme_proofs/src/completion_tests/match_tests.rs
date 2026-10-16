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

//! An entry completes a command only when it names the command in flight
//! and the queue it was issued on; one naming anything else is consumed.

use super::ring::{entry, ok_status, RINGS};
use super::wait::wait_on;
use crate::admin::CqCursor;
use crate::error::NvmeError;

#[test]
fn an_entry_for_another_command_does_not_complete_this_one() {
    for entries in RINGS {
        for other in [0u16, 1, 6, 8, 0x8007, 0xffff] {
            let mut ring = vec![entry(0, 0, 0, ok_status(false)); entries as usize];
            ring[0] = entry(other, 0, 0, ok_status(true));
            let mut c = CqCursor::new(entries);
            let w = wait_on(&mut c, 0, 7, &ring, 2);
            assert!(
                matches!(w.result, Err(NvmeError::ControllerTimeout)),
                "cid {other} completed command 7"
            );
            // The stray entry is consumed, and only it.
            assert_eq!(w.rung, vec![1 % entries]);
        }
    }
}

#[test]
fn an_entry_from_another_submission_queue_does_not_complete_this_one() {
    // The admin queue serves SQ 0 and the I/O queue SQ 1. A phase-correct,
    // successful entry with the right command id but another SQ id is not
    // this command's completion.
    for entries in RINGS {
        for (expect, other) in [(0u16, 1u16), (0, 0xffff), (1, 0), (1, 2), (1, 0x8001)] {
            let ring = vec![entry(4, other, 0, ok_status(true)); entries as usize];
            let mut c = CqCursor::new(entries);
            let w = wait_on(&mut c, expect, 4, &ring, 2);
            assert!(
                matches!(w.result, Err(NvmeError::ControllerTimeout)),
                "SQ {other} completed a command issued on SQ {expect}"
            );
            // Every stray entry of this pass is consumed, then the ring is
            // empty until the next pass.
            assert_eq!(w.rung.len(), entries as usize);
            let ring = vec![entry(4, expect, 0, ok_status(true)); entries as usize];
            let mut c = CqCursor::new(entries);
            assert!(wait_on(&mut c, expect, 4, &ring, 2).result.is_ok());
        }
    }
}
