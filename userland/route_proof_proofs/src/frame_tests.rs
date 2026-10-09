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

use crate::attest_protocol as attest;
use crate::board::{read_answer, Board, ANSWER_LEN};
use crate::frame::{
    ask_frame, post_frame, reply_body, ANSWER_FRAME_LEN, ATTEST_HDR_LEN, ATTEST_MAGIC,
    ATTEST_VERSION, OP_PROOF_ROUTE, OP_ROUTE_REPORT,
};
use crate::report::decode;
use crate::report_tests::{ready_anyone, ready_nym};

#[test]
fn the_frame_constants_are_the_attest_services() {
    assert_eq!(ATTEST_MAGIC, attest::MAGIC);
    assert_eq!(ATTEST_VERSION, attest::VERSION);
    assert_eq!(ATTEST_HDR_LEN, attest::HDR_LEN);
    assert_eq!(OP_ROUTE_REPORT, attest::OP_ROUTE_REPORT);
    assert_eq!(OP_PROOF_ROUTE, attest::OP_PROOF_ROUTE);
}

#[test]
fn the_attest_decoder_reads_a_post_back_to_the_report() {
    let frame = post_frame(&ready_anyone(), 77);
    let (req, payload) = attest::parse(&frame).ok().expect("attest refused the post");
    assert_eq!(req.op, OP_ROUTE_REPORT);
    assert_eq!(req.request_id, 77);
    assert_eq!(decode(payload), Some(ready_anyone()));
}

#[test]
fn the_attest_decoder_reads_a_question_with_no_payload() {
    let frame = ask_frame(9);
    let (req, payload) = attest::parse(&frame).ok().expect("attest refused the question");
    assert_eq!(req.op, OP_PROOF_ROUTE);
    assert_eq!(req.request_id, 9);
    assert!(payload.is_empty());
}

fn attest_reply(op: u16, request_id: u32, status: i32, body: &[u8]) -> std::vec::Vec<u8> {
    let req = attest::Request { op, flags: 0, request_id };
    let mut out = std::vec![0u8; attest::HDR_LEN + 4 + body.len()];
    attest::response_header(&mut out, &req, (4 + body.len()) as u32);
    attest::write_status(&mut out, status);
    out[attest::HDR_LEN + 4..].copy_from_slice(body);
    out
}

#[test]
fn an_answer_built_by_the_attest_encoder_reads_back_whole() {
    let mut board = Board::new();
    board.post(b"net.nym", crate::authorize::CAP_NETWORK, &crate::report::encode(&ready_nym()), 10).unwrap();
    let answer = board.answer(30);
    let reply = attest_reply(OP_PROOF_ROUTE, 5, 0, &answer);
    assert_eq!(reply.len(), ANSWER_FRAME_LEN);
    let (status, body) = reply_body(&reply, OP_PROOF_ROUTE, 5).unwrap();
    assert_eq!(status, 0);
    assert_eq!(body.len(), ANSWER_LEN);
    assert_eq!(read_answer(body), Some((Some((ready_nym(), 20)), None)));
}

#[test]
fn a_reply_to_something_else_is_not_taken_for_the_answer() {
    let body = [0u8; ANSWER_LEN];
    let reply = attest_reply(OP_PROOF_ROUTE, 5, 0, &body);
    assert!(reply_body(&reply, OP_PROOF_ROUTE, 6).is_none(), "wrong request id");
    assert!(reply_body(&reply, OP_ROUTE_REPORT, 5).is_none(), "wrong op");
    let mut bad = reply.clone();
    bad[0] ^= 1;
    assert!(reply_body(&bad, OP_PROOF_ROUTE, 5).is_none(), "wrong magic");
    // A payload length past the bytes received.
    let mut long = reply.clone();
    long[16..20].copy_from_slice(&((4 + ANSWER_LEN + 1) as u32).to_le_bytes());
    assert!(reply_body(&long, OP_PROOF_ROUTE, 5).is_none());
    long[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(reply_body(&long, OP_PROOF_ROUTE, 5).is_none());
    // Shorter than a header and a status.
    for n in 0..attest::HDR_LEN + 4 {
        assert!(reply_body(&reply[..n], OP_PROOF_ROUTE, 5).is_none());
    }
}

#[test]
fn a_refusal_carries_its_status_and_no_body() {
    let reply = attest_reply(OP_ROUTE_REPORT, 1, -1, &[]);
    assert_eq!(reply_body(&reply, OP_ROUTE_REPORT, 1), Some((-1, &[][..])));
}
