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

// A completion that arrives after its own wait timed out is consumed by the
// wait for a later command, which still finishes its own: left at the head
// it would fail every later command on the queue.

use crate::admin::{wait_for_completion, CqCursor};
use crate::completion_tests::{entry, ok_status, RINGS};

#[test]
fn a_late_completion_is_consumed_and_the_wait_finishes_its_own() {
    for entries in RINGS.into_iter().filter(|&e| e >= 2) {
        let mut ring = vec![entry(0, 0, 0, ok_status(false)); entries as usize];
        ring[0] = entry(6, 1, 0, ok_status(true));
        ring[1] = entry(7, 1, 0, ok_status(true));
        let mut cursor = CqCursor::new(entries);
        let mut heads = Vec::new();
        let done = wait_for_completion(
            &mut cursor,
            1,
            7,
            |head| ring[head as usize],
            |head| heads.push(head),
            || false,
        );
        assert!(done.is_ok(), "ring of {entries}: the wait finishes its own command");
        assert_eq!(heads, [1, 2 % entries], "ring of {entries}: both consumed, in order");
    }
}

#[test]
fn a_wait_that_runs_out_has_consumed_what_it_saw() {
    let mut ring = [entry(0, 0, 0, ok_status(false)); 8];
    ring[0] = entry(6, 1, 0, ok_status(true));
    let mut cursor = CqCursor::new(8);
    let (mut heads, mut checks) = (Vec::new(), 0u32);
    let done = wait_for_completion(
        &mut cursor,
        1,
        7,
        |head| ring[head as usize],
        |head| heads.push(head),
        || {
            checks += 1;
            checks > 2
        },
    );
    assert!(done.is_err(), "nothing completed command 7");
    assert_eq!((heads, cursor.head), (vec![1], 1));
}
