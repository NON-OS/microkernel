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

//! OP_INSTALL_READY's body: the verdict, then one byte a gate, eleven gates.

use nonos_market_proto::{parse_readiness, GATES};

#[test]
fn readiness_is_the_verdict_then_one_byte_a_gate() {
    let r = parse_readiness(&[1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1]).unwrap();
    assert!(r.install_ready);
    assert_eq!(r.gates, [true, false, true, true, true, false, true, false, true, false, true]);
    assert!(parse_readiness(&[1; 11]).is_none(), "a reply without the bond byte");
}

#[test]
fn the_schema_3_gates_follow_the_first_six() {
    assert_eq!(GATES.len(), 11);
    let v3 = [&b"commerce"[..], b"release attest", b"guest policy", b"price", b"bond"];
    assert_eq!(GATES[6..], v3);
}

#[test]
fn the_request_ends_with_the_chain_the_bond_is_read_on() {
    let b = nonos_market_proto::ready_body(b"linux.a", b"", 11_155_111);
    assert_eq!(b[..b.len() - 8], nonos_market_proto::pair_body(b"linux.a", b"")[..]);
    assert_eq!(b[b.len() - 8..], 11_155_111u64.to_le_bytes());
}
