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

use super::reply_rule::pick;

/// Each entry is (inbox, token, dequeued).
fn at(entries: &[(&'static str, u64, bool)], inbox: &str, served: u64) -> Option<usize> {
    pick(entries.iter().copied(), inbox, served)
}

#[test]
fn the_served_entry_wins_over_an_older_one_of_the_same_caller() {
    // Call 7 was taken and never answered; the server serves 9.
    let q = [("c", 7, true), ("d", 8, false), ("c", 9, true)];
    assert_eq!(at(&q, "c", 9), Some(2));
}

#[test]
fn a_served_token_of_another_caller_leaves_this_one_its_oldest_dequeued() {
    // A server took c's 7, then d's 8, and answers c first, in order.
    let q = [("c", 7, true), ("d", 8, true)];
    assert_eq!(at(&q, "c", 8), Some(0));
    assert_eq!(at(&q, "d", 8), Some(1));
}

#[test]
fn a_request_still_in_the_inbox_is_never_answered() {
    // c's 9 was not dequeued: a reply to c answers nothing, whatever was served.
    let q = [("c", 9, false)];
    assert_eq!(at(&q, "c", 0), None);
    assert_eq!(at(&q, "c", 9), None, "the served token must also be dequeued");
    assert_eq!(at(&[("c", 7, false), ("c", 9, true)], "c", 0), Some(1));
}

#[test]
fn a_reply_no_call_is_owed_is_dropped() {
    assert_eq!(at(&[("d", 8, true)], "c", 8), None);
    assert_eq!(at(&[], "c", 0), None);
}
