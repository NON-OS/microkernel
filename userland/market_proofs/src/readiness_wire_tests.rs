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

//! OP_INSTALL_READY's body: the verdict, one byte a gate for eleven gates,
//! then the bond's code (v1.4.35), 0 being Unread.

use nonos_market_proto::{parse_readiness, BondCode, GATES, READINESS_LEN};

#[test]
fn readiness_is_the_verdict_then_one_byte_a_gate() {
    let r = parse_readiness(&[1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 2]).unwrap();
    assert!(r.install_ready);
    assert_eq!(r.gates, [true, false, true, true, true, false, true, false, true, false, true]);
    assert_eq!(r.bond, BondCode::Active);
    assert_eq!(READINESS_LEN, 13);
    assert!(parse_readiness(&[1; 11]).is_none(), "a reply without the bond byte");
    assert!(parse_readiness(&[1; 12]).is_none(), "a reply without the bond's code");
    assert!(parse_readiness(&[[1; 12].as_slice(), &[7]].concat()).is_none(), "no code 7");
}

#[test]
fn a_zero_code_is_unread_and_each_failure_has_its_own_sentence() {
    let r = parse_readiness(&[0; 13]).unwrap();
    assert_eq!(r.bond, BondCode::Unread);
    let codes: Vec<BondCode> = (0..7).map(|b| BondCode::from_byte(b).unwrap()).collect();
    assert!(codes.iter().enumerate().all(|(i, c)| *c as usize == i), "byte = code");
    let said: Vec<&[u8]> = codes.iter().filter_map(|c| c.sentence()).collect();
    assert_eq!(said.len(), 5, "NotNeeded and Active pass: no sentence");
    for (i, a) in said.iter().enumerate() {
        assert!(said[i + 1..].iter().all(|b| a != b), "sentence {i} is its own");
    }
    let unread = BondCode::Unread.sentence().unwrap();
    assert_eq!(unread, b"Seller's bond could not be read");
    let read = codes.iter().filter(|c| **c != BondCode::Unread);
    assert!(read.filter_map(|c| c.sentence()).all(|s| !s.windows(4).any(|w| w == b"read")));
}

#[test]
fn the_schema_3_gates_follow_the_first_six() {
    assert_eq!(GATES.len(), 11);
    let v3 = [&b"commerce"[..], b"release attest", b"guest policy", b"price", b"bond"];
    assert_eq!(GATES[6..], v3);
}

#[test]
fn the_request_ends_with_the_chain_the_bond_is_read_on() {
    let b = nonos_market_proto::ready_body(b"linux.a", b"", 11_155_111).unwrap();
    assert_eq!(b[..b.len() - 8], nonos_market_proto::pair_body(b"linux.a", b"").unwrap()[..]);
    assert_eq!(b[b.len() - 8..], 11_155_111u64.to_le_bytes());
}
