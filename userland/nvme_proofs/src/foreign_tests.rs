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

// The I/O queue knows a command it gave up on is finished only when the wait
// for a later command consumes its completion. The wait names each one it
// consumes for another command, and still finishes its own.

use crate::admin::{wait_noting_foreign, CqCursor};
use crate::completion_tests::{entry, ok_status, RINGS};

#[test]
fn a_late_completion_is_named_and_the_wait_finishes_its_own() {
    for entries in RINGS.into_iter().filter(|&e| e >= 2) {
        let mut ring = vec![entry(0, 0, 0, ok_status(false)); entries as usize];
        ring[0] = entry(6, 1, 0, ok_status(true));
        ring[1] = entry(7, 1, 0, ok_status(true));
        let mut cursor = CqCursor::new(entries);
        let mut seen = Vec::new();
        let done = wait_noting_foreign(
            &mut cursor,
            1,
            7,
            |head| ring[head as usize],
            |_| {},
            || false,
            |cid| seen.push(cid),
        );
        assert!(done.is_ok(), "ring of {entries}: the wait finishes its own command");
        assert_eq!(seen, [6], "ring of {entries}: the late completion is named once");
    }
}

#[test]
fn a_wait_that_runs_out_names_what_it_consumed() {
    let mut ring = [entry(0, 0, 0, ok_status(false)); 8];
    ring[0] = entry(6, 1, 0, ok_status(true));
    let mut cursor = CqCursor::new(8);
    let mut seen = Vec::new();
    let mut checks = 0u32;
    let done = wait_noting_foreign(
        &mut cursor,
        1,
        7,
        |head| ring[head as usize],
        |_| {},
        || {
            checks += 1;
            checks > 2
        },
        |cid| seen.push(cid),
    );
    assert!(done.is_err(), "nothing completed command 7");
    assert_eq!(seen, [6]);
}
