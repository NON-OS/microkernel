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

//! A whole record: the header placed, the entries in a dense order with
//! times that never go back, and a footer that agrees with them.

use crate::build::record::RecordB;
use nsh_wire::{EntryBody, Record, WireError};

pub fn parse(b: &RecordB) -> Result<(), WireError> {
    Record::parse(&b.bytes()).map(|_| ())
}

#[test]
fn a_record_places_its_header_entries_and_footer() {
    let bytes = RecordB::default().bytes();
    let r = Record::parse(&bytes).expect("valid");
    assert_eq!(
        (r.header.user, r.header.role, r.header.start_ms),
        (&b"ek"[..], &b"auditor"[..], 1_000)
    );
    assert_eq!(r.header.caps.len(), 2);
    let bodies: Vec<_> = r.entries().map(|e| (e.seq, e.body)).collect();
    assert_eq!(bodies, [(0, EntryBody::Command(b"ls /var/log")), (1, EntryBody::Exit(0))]);
    assert_eq!((r.footer.count, r.footer.end_ms, r.footer.end_reason), (2, 1_300, 1));
    assert_eq!(r.signed, &bytes[..bytes.len() - 3373]);
    assert_eq!(&bytes[..r.header.bytes.len()], r.header.bytes);
    assert_eq!((r.header.transcript, r.header.host.ed), (&[0x7A; 64], &[0x4B; 32]));
    assert_eq!(r.header.enrol.host, b"db-1.eu.example");
}

#[test]
fn a_header_names_at_least_one_capability() {
    assert_eq!(parse(&RecordB { caps: vec![], ..RecordB::default() }), Err(WireError::Value));
}

#[test]
fn a_record_with_no_entries_is_whole() {
    assert_eq!(parse(&RecordB { entries: vec![], ..RecordB::default() }), Ok(()));
}

#[test]
fn a_gap_a_repeat_or_a_second_start_in_the_sequence_is_refused() {
    for seqs in [[1, 2], [0, 0], [0, 2], [1, 0]] {
        let mut b = RecordB::default();
        b.entries.iter_mut().zip(seqs).for_each(|(e, s)| e.seq = s);
        assert_eq!(parse(&b), Err(WireError::Value), "{seqs:?}");
    }
}

#[test]
fn time_never_goes_back() {
    let mut b = RecordB::default();
    b.entries[1].at = 1_099;
    assert_eq!(parse(&b), Err(WireError::Value));
    b.entries[1].at = 1_100;
    assert_eq!(parse(&b), Ok(()));
    b.entries[0].at = 999;
    assert_eq!(parse(&b), Err(WireError::Value), "an entry before the start");
    assert_eq!(parse(&RecordB { end: 1_199, ..RecordB::default() }), Err(WireError::Value));
}
