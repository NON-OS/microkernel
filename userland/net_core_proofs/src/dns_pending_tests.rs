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

//! The waiting DNS lookups: a full table refuses at once, a caller who calls
//! again is answered for the lookup it gave up on first, and no lookup waits
//! past its deadline whatever its socket says.

use crate::dns_pending::{Look, Table, Waiting, PENDING_MAX};

fn w(pid: u32, request_id: u32, query: u16, deadline_ms: i64) -> Waiting<u16> {
    Waiting { pid, request_id, query, deadline_ms }
}

#[test]
fn an_answer_frees_the_slot_and_a_pending_lookup_stays() {
    let mut t = Table::new();
    t.add(w(10, 1, 100, 3_000)).unwrap();
    t.add(w(11, 2, 101, 3_000)).unwrap();
    let mut looked = Vec::new();
    t.sweep(5, |q, expired| {
        assert!(!expired);
        looked.push(q.query);
        if q.query == 100 { Look::Answered } else { Look::Pending }
    });
    assert_eq!(looked, [100, 101]);
    assert_eq!(t.take_for(10), None, "answered and gone");
    assert_eq!(t.take_for(11), Some(w(11, 2, 101, 3_000)), "still waiting");
    assert!(!t.any());
}

#[test]
fn a_lookup_never_waits_past_its_deadline() {
    let mut t = Table::new();
    t.add(w(10, 1, 100, 3_000)).unwrap();
    let mut seen = None;
    // The socket still says pending, but the deadline has come: the caller
    // is told (look is called with expired) and the slot is freed regardless.
    t.sweep(3_000, |_, expired| {
        seen = Some(expired);
        Look::Pending
    });
    assert_eq!(seen, Some(true));
    assert!(!t.any());
}

#[test]
fn a_full_table_hands_the_lookup_back_to_be_refused() {
    let mut t = Table::new();
    for i in 0..PENDING_MAX as u32 {
        t.add(w(100 + i, i, i as u16, 3_000)).unwrap();
    }
    let extra = w(999, 7, 77, 3_000);
    assert_eq!(t.add(extra), Err(extra));
    t.sweep(0, |q, _| if q.pid == 100 { Look::Answered } else { Look::Pending });
    assert!(t.add(extra).is_ok(), "a freed slot takes the next one");
}

// A caller that gave up on a lookup and calls again: the kernel delivers a
// caller's replies to its calls in order, so the loop takes the old lookup out
// (and answers it) before dispatching the new call, and only that caller's.
#[test]
fn a_caller_calling_again_is_answered_for_the_old_lookup_first() {
    let mut t = Table::new();
    t.add(w(10, 1, 100, 3_000)).unwrap();
    t.add(w(11, 2, 101, 3_000)).unwrap();
    assert_eq!(t.take_for(10).map(|q| q.request_id), Some(1));
    assert_eq!(t.take_for(10), None, "owed once");
    let mut left = Vec::new();
    t.sweep(0, |q, _| {
        left.push(q.pid);
        Look::Pending
    });
    assert_eq!(left, [11], "the other caller still waits");
}
