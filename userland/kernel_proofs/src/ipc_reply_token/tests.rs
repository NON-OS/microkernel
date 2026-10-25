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

fn at(entries: &[(&'static str, u64)], inbox: &str, served: u64) -> Option<usize> {
    pick(entries.iter().copied(), inbox, served)
}

#[test]
fn the_served_entry_wins_over_an_older_one_of_the_same_caller() {
    // Call 7 was never answered and keeps its entry; the server serves 9.
    let q = [("c", 7), ("d", 8), ("c", 9)];
    assert_eq!(at(&q, "c", 9), Some(2));
}

#[test]
fn a_served_token_of_another_caller_leaves_this_one_its_oldest() {
    // A server took c's 7, then d's 8, and answers c first, in order.
    let q = [("c", 7), ("d", 8)];
    assert_eq!(at(&q, "c", 8), Some(0));
    assert_eq!(at(&q, "d", 8), Some(1));
}

#[test]
fn with_nothing_served_the_oldest_answers() {
    assert_eq!(at(&[("c", 7), ("c", 9)], "c", 0), Some(0));
}

#[test]
fn a_reply_no_call_is_owed_is_dropped() {
    assert_eq!(at(&[("d", 8)], "c", 8), None);
    assert_eq!(at(&[], "c", 0), None);
}
