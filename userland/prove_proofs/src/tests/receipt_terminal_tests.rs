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

//! The Terminal writes the receipt frames without the prover's crate
//! (capsule_terminal receipt_frame.rs): every byte it sends is the one
//! nonos_proof_abi::wire writes, every reply it reads is one that crate
//! writes (receipt_terminal_ack_tests.rs: the seal number it acks).

use nonos_proof_abi::wire::{self, Header, ReceiptRequest, PROVE_MAGIC, RECEIPT, RECEIPT_ACK};

use crate::terminal_receipt_frame as t;

#[test]
fn the_terminal_writes_the_provers_frames() {
    assert_eq!((t::PROVE_MAGIC, t::RECEIPT, t::RECEIPT_ACK), (PROVE_MAGIC, RECEIPT, RECEIPT_ACK));
    assert_eq!((t::HDR_LEN, t::FRAME_MAX), (wire::HDR_LEN, wire::FRAME_MAX));
    let policy = [0x5a; 256];
    let body = t::receipt_body(b"allow:/tmp/a,net:10.0.2.2:80", &policy).unwrap();
    let req = ReceiptRequest { box_word: b"allow:/tmp/a,net:10.0.2.2:80", policy: &policy };
    assert_eq!(Some(body.clone()), req.encode());
    assert_eq!(t::request(RECEIPT, 77, &body), wire::request(PROVE_MAGIC, RECEIPT, 77, &body));
    assert_eq!(
        (t::receipt_body(b"", &policy), t::receipt_body(&[b'a'; 256], &policy)),
        (None, None)
    );
}

#[test]
fn the_terminal_reads_the_provers_replies_and_no_other() {
    let h = Header { op: RECEIPT, flags: 0, id: 5, len: 0 };
    let rx = wire::reply(PROVE_MAGIC, &h, 0, b"napr");
    assert_eq!(t::reply(&rx, RECEIPT, 5), Some((0, &b"napr"[..])));
    let rx = wire::reply(PROVE_MAGIC, &h, -22, b"");
    assert_eq!(t::reply(&rx, RECEIPT, 5), Some((-22, &b""[..])));
    assert_eq!(t::reply(&rx, RECEIPT, 6), None, "another id");
    assert_eq!(t::reply(&rx, RECEIPT_ACK, 5), None, "another op");
    assert_eq!(t::reply(&rx[..rx.len() - 1], RECEIPT, 5), None, "short");
    assert_eq!(t::reply(&[&rx[..], b"x"].concat(), RECEIPT, 5), None, "long");
    let other = wire::reply(0x4D4B_5450, &h, 0, b"");
    assert_eq!(t::reply(&other, RECEIPT, 5), None, "another magic");
}
