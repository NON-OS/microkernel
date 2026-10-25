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

//! A command entry in a record holds one to four KiB.

use crate::build::record::{EntryB, RecordB};
use nsh_wire::record::MAX_COMMAND;

#[test]
fn a_command_is_one_to_four_kib_in_a_record() {
    let entry = |n: usize| EntryB { seq: 0, at: 1_100, kind: 1, body: vec![b'x'; n] };
    for (n, ok) in [(1, true), (MAX_COMMAND, true), (MAX_COMMAND + 1, false), (0, false)] {
        let b = RecordB { entries: vec![entry(n)], ..RecordB::default() };
        assert_eq!(super::record::parse(&b).is_ok(), ok, "{n}");
    }
}

#[test]
fn a_record_over_sixty_four_mib_is_refused_before_it_is_read() {
    let big = vec![0u8; nsh_wire::record::MAX_RECORD + 1];
    assert_eq!(nsh_wire::Record::parse(&big), Err(nsh_wire::WireError::Oversize));
}

#[test]
fn a_record_holds_at_most_two_to_the_twenty_entries() {
    let exit = |seq| EntryB { seq, at: 1_100, kind: 7, body: vec![0; 4] };
    let n = nsh_wire::record::MAX_RECORD_ENTRIES;
    let full = RecordB { entries: (0..n).map(exit).collect(), ..RecordB::default() };
    assert_eq!(super::record::parse(&full), Ok(()));
    let mut over = full;
    over.entries.push(exit(n));
    assert_eq!(super::record::parse(&over), Err(nsh_wire::WireError::Oversize));
}
