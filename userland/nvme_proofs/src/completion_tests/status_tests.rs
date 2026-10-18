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

//! What the driver reads from its own command's entry: any status bit fails
//! the command, and the SQ head the controller reports never moves the
//! cursor.

use super::ring::{cursor_at, entry, ok_status, RINGS};
use super::wait::wait_on;
use crate::error::NvmeError;

#[test]
fn any_status_bit_fails_the_command() {
    for entries in RINGS {
        for phase in [true, false] {
            let head = entries - 1;
            for bit in 1..16 {
                let ring = vec![entry(3, 0, 0, ok_status(phase) | (1 << bit)); entries as usize];
                let mut c = cursor_at(entries, head, phase);
                let w = wait_on(&mut c, 0, 3, &ring, 2);
                assert!(
                    matches!(w.result, Err(NvmeError::AdminCommandFailed)),
                    "status bit {bit} passed"
                );
                // A failed command is still consumed, so the queue moves on.
                assert_eq!(w.rung, vec![0]);
            }
            let ring = vec![entry(3, 0, 0, ok_status(phase)); entries as usize];
            let mut c = cursor_at(entries, head, phase);
            assert!(wait_on(&mut c, 0, 3, &ring, 2).result.is_ok());
        }
    }
}

#[test]
fn the_reported_sq_head_never_moves_the_cursor() {
    for entries in RINGS {
        for head in 0..entries {
            for sq_head in [0, 1, entries - 1, entries, 0x7fff, 0xfffe, 0xffff] {
                let ring = vec![entry(5, 0, sq_head, ok_status(true)); entries as usize];
                let mut c = cursor_at(entries, head, true);
                let w = wait_on(&mut c, 0, 5, &ring, 2);
                assert!(w.result.is_ok());
                let wrapped = head + 1 == entries;
                assert_eq!(c.head, if wrapped { 0 } else { head + 1 }, "sq_head {sq_head}");
                assert_eq!(c.phase, !wrapped, "the phase flips on the wrap only");
                assert_eq!(w.rung, vec![c.head]);
            }
        }
    }
}
