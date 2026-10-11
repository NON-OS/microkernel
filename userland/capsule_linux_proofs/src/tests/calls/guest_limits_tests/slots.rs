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

//! Tables that reuse a free slot before they grow, and symlink's ceiling.

use crate::linux::guest::links_room::{room, MAX_LINKS};
use crate::linux::guest::slots::{pick, MAX_SLOTS};

#[test]
fn a_slot_nothing_names_is_taken_before_the_table_grows() {
    assert_eq!(pick(0, |_| false), Some(0));
    assert_eq!(pick(3, |_| false), Some(3));
    assert_eq!(pick(3, |i| i == 1), Some(1));
    assert_eq!(pick(3, |i| i >= 1), Some(1), "the lowest free slot");
    assert_eq!(pick(MAX_SLOTS, |_| false), None, "a full table is ENFILE");
    assert_eq!(pick(MAX_SLOTS + 9, |i| i == MAX_SLOTS + 1), None, "nothing past the ceiling");
}

/// A guest that makes and closes a pipe, or is refused one, a hundred
/// thousand times: the table stays the size of what it holds at once.
#[test]
fn making_and_closing_forever_never_grows_the_table() {
    let mut named: Vec<bool> = Vec::new();
    for round in 0..100_000 {
        let Some(slot) = pick(named.len(), |i| !named[i]) else {
            panic!("refused at round {round}");
        };
        match named.get_mut(slot) {
            Some(n) => *n = true,
            None => named.push(true),
        }
        // Every third one is kept open a while; the rest close at once.
        if round % 3 != 0 || round % 30 == 0 {
            named[slot] = false;
        }
        if round % 30 == 0 {
            named.iter_mut().for_each(|n| *n = false);
        }
    }
    assert!(named.len() <= 12, "the table grew to {}", named.len());
}

#[test]
fn symlink_stops_at_the_ceiling_as_a_full_filesystem_does() {
    assert!(room(0) && room(MAX_LINKS - 1));
    assert!(!room(MAX_LINKS), "one more is ENOSPC");
    // A guest making links in a loop stops at the ceiling, however long it runs.
    let mut table = 0usize;
    for _ in 0..(MAX_LINKS * 3) {
        if room(table) {
            table += 1;
        }
    }
    assert_eq!(table, MAX_LINKS);
}
