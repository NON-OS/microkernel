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

//! The rule: a dead server names every call it owed, a dead caller leaves
//! every queue, and an abandoned call is told once, a gone server refused.

use super::abandoned::{forget, gone, record, take};
use super::death::{on_death, Queues};
use super::entry::Entry;
use alloc::collections::VecDeque;
use alloc::string::String;

fn q(entries: &[(u32, &str, u64)]) -> VecDeque<Entry> {
    entries.iter().map(|(p, i, t)| Entry::new(*p, String::from(*i), *t)).collect()
}

#[test]
fn a_dead_server_names_every_call_it_owed_oldest_first() {
    let mut queues = Queues::new();
    queues.insert(7, q(&[(20, "r20", 1), (21, "r21", 2), (20, "r20", 3)]));
    queues.insert(8, q(&[(20, "r20", 4)]));
    assert_eq!(on_death(&mut queues, 7, "r7"), [(20, 1), (21, 2), (20, 3)]);
    assert!(!queues.contains_key(&7));
    assert_eq!(queues[&8], q(&[(20, "r20", 4)]), "another server's calls stay");
}

#[test]
fn a_dead_caller_leaves_every_queue_and_is_owed_nothing() {
    let mut queues = Queues::new();
    queues.insert(7, q(&[(30, "r30", 1), (31, "r31", 2)]));
    queues.insert(8, q(&[(30, "r30", 3)]));
    assert_eq!(on_death(&mut queues, 30, "r30"), []);
    assert_eq!(queues[&7], q(&[(31, "r31", 2)]));
    assert!(!queues.contains_key(&8), "an emptied queue goes");
    // A server calling itself and dying is owed nothing either.
    queues.insert(9, q(&[(9, "r9", 5), (32, "r32", 6)]));
    assert_eq!(on_death(&mut queues, 9, "r9"), [(32, 6)]);
}

#[test]
fn a_caller_already_waiting_is_told_once_and_a_gone_server_is_refused() {
    record(40, &[(41, 9), (42, 10)]);
    assert!(gone(40) && !gone(41));
    assert!(take(41, 9), "the waiting call returns at once");
    assert!(!take(41, 9), "and only once");
    assert!(!take(41, 10), "another token is not abandoned");
    forget(42);
    assert!(!take(42, 10), "a freed caller's entries go");
    forget(40);
    assert!(!gone(40), "a freed server is forgotten");
}
