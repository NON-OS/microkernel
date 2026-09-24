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

//! The relay entries, and the field readers they are built from.

extern crate alloc;

use crate::directory::consensus::parse;
use crate::directory::{decimal, ipv4, lines, port};
use crate::vectors::CONSENSUS;

#[test]
fn the_first_relay_is_read_exactly() {
    let doc = parse(CONSENSUS).expect("parses");
    let first = &doc.entries[0];
    assert_eq!(first.or_port, 9_001, "relays advertise 9001, not the authority 9201");
    assert_ne!(first.weight, 0, "a relay with no published weight is unusable");
    assert!(first.microdesc_digest.iter().any(|b| *b != 0));
    assert!(first.flags.running && first.flags.valid);
}

#[test]
fn field_readers_refuse_what_they_cannot_represent() {
    assert_eq!(ipv4(b"150.40.119.9"), Some([150, 40, 119, 9]));
    assert_eq!(ipv4(b"150.40.119"), None);
    assert_eq!(ipv4(b"150.40.119.9.1"), None);
    assert_eq!(ipv4(b"150.40.119.256"), None);
    assert_eq!(ipv4(b"2001:db8::1"), None, "an IPv6 address is not truncated");
    assert_eq!(port(b"9001"), Some(9_001));
    assert_eq!(port(b"0"), None, "port zero is not reachable");
    assert_eq!(port(b"65536"), None);
    assert_eq!(decimal(b""), None);
    assert_eq!(decimal(b"18446744073709551616"), None, "past u64 must not wrap");
}

#[test]
fn a_document_that_is_not_the_microdesc_flavour_is_refused() {
    let swapped = replace_first(
        CONSENSUS,
        b"network-status-version 3 microdesc",
        b"network-status-version 3          ",
    );
    assert!(parse(&swapped).is_none(), "a plain network-status must not be accepted");
}

#[test]
fn line_walking_reaches_a_last_line_with_no_newline() {
    assert_eq!(lines(b"a 1\nb 2\nc 3").count(), 3);
    assert_eq!(lines(b"").count(), 0);
}

fn replace_first(body: &[u8], from: &[u8], to: &[u8]) -> alloc::vec::Vec<u8> {
    let at = (0..body.len() - from.len())
        .find(|i| &body[*i..*i + from.len()] == from)
        .expect("marker present");
    let mut out = body.to_vec();
    out[at..at + to.len()].copy_from_slice(to);
    out
}
