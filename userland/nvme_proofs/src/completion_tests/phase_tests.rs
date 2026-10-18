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

//! An entry is taken only when its phase tag is this pass's: one left from
//! the last pass never is, and a controller writing around the ring pass
//! after pass completes every command.

use super::ring::{cursor_at, entry, ok_status, RINGS};
use super::wait::wait_on;
use crate::admin::CqCursor;
use crate::error::NvmeError;

#[test]
fn an_entry_with_last_passes_phase_is_never_taken() {
    for entries in RINGS {
        for head in 0..entries {
            for phase in [true, false] {
                // Every slot holds a perfect completion for this command, but
                // tagged with the other phase: written on the previous pass.
                let ring = vec![entry(9, 0, head, ok_status(!phase)); entries as usize];
                let mut c = cursor_at(entries, head, phase);
                let w = wait_on(&mut c, 0, 9, &ring, 2);
                assert!(matches!(w.result, Err(NvmeError::ControllerTimeout)));
                assert!(w.rung.is_empty(), "a stale entry was consumed");
                assert_eq!((c.head, c.phase), (head, phase));
            }
        }
    }
}

#[test]
fn a_well_behaved_controller_completes_around_the_ring() {
    for entries in RINGS {
        let mut ring = vec![entry(0, 0, 0, ok_status(false)); entries as usize];
        let (mut tail, mut phase) = (0u16, true);
        let mut c = CqCursor::new(entries);
        for n in 0..(3 * entries as u32 + 1) {
            let cid = (n % 0xfffe + 1) as u16;
            ring[tail as usize] = entry(cid, 0, tail, ok_status(phase));
            tail += 1;
            if tail == entries {
                tail = 0;
                phase = !phase;
            }
            let w = wait_on(&mut c, 0, cid, &ring, 2);
            assert!(w.result.is_ok(), "command {n} of a {entries}-entry ring");
            assert_eq!((c.head, c.phase), (tail, phase));
            assert_eq!(w.rung, vec![tail]);
        }
    }
}
