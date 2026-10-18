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

//! Malformed WDAT: unknown instruction kinds and register spaces dropped
//! and counted, counts past the table or the cap, a short table.

use super::build::{entry, table, IO};
use super::wdat_action::*;
use super::wdat_entry::{wdat, MAX_ENTRIES};

#[test]
fn malformed_entries_dropped_and_counted() {
    let e = [
        entry(RESET, 4, IO, 0x10, 1, 1),
        entry(RESET, 2, 2, 0x10, 1, 1),
        entry(RESET, 2, 0x7F, 0x10, 1, 1),
        entry(RESET, 2, IO, 0x10, 1, 1),
    ];
    let w = wdat(&table(&e, 4)).unwrap();
    assert_eq!((w.entries.len(), w.malformed), (1, 3));
}

#[test]
fn counts_past_the_bytes_or_the_cap() {
    let e = [entry(RESET, 2, IO, 0x10, 1, 1), entry(QUERY_RUNNING, 0, IO, 0x14, 1, 1)];
    assert_eq!(wdat(&table(&e, 9)).unwrap().entries.len(), 2, "count past the table");
    assert_eq!(wdat(&table(&e, 1)).unwrap().entries.len(), 1, "count below the bytes");
    let mut t = table(&e, 2);
    t.truncate(t.len() - 3);
    assert_eq!(wdat(&t).unwrap().entries.len(), 1, "a partial entry is not read");
    let many: Vec<_> = (0..200).map(|_| entry(RESET, 2, IO, 0x10, 1, 1)).collect();
    assert_eq!(wdat(&table(&many, 200)).unwrap().entries.len(), MAX_ENTRIES);
}

#[test]
fn a_short_table_has_no_watchdog() {
    let t = table(&[], 0);
    assert!(wdat(&t[..60]).is_none());
    assert!(wdat(&[]).is_none());
}
