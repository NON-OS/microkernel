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

//! Identity request framing and reply decoding.

use crate::term::identity::{decode_str, request, REQ_LEN};
use nonos_policy_proto::{Header, E_OK, HDR_LEN, KIND_STR, OP_GET};

const HOSTNAME: u32 = 0x0301;

fn reply(op: u16, field: u32, kind: u8, status: u16, body: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8; HDR_LEN];
    let hdr = Header { op, field, kind, status, payload_len: body.len() as u16 };
    hdr.encode(&mut v[..HDR_LEN]);
    v.extend_from_slice(body);
    v
}

#[test]
fn the_request_is_a_bare_header_naming_the_field() {
    let mut tx = [0u8; REQ_LEN];
    request(HOSTNAME, &mut tx);
    let h = Header::decode(&tx).unwrap();
    assert_eq!((h.op, h.field, h.kind, h.payload_len), (OP_GET, HOSTNAME, KIND_STR, 0));
}

#[test]
fn a_matching_reply_yields_the_string_after_the_header() {
    let rx = reply(OP_GET, HOSTNAME, KIND_STR, E_OK, b"station");
    assert_eq!(decode_str(HOSTNAME, &rx), Some(&b"station"[..]));
}

#[test]
fn a_reply_that_answers_a_different_question_is_refused() {
    let ok = E_OK;
    assert_eq!(decode_str(HOSTNAME, &reply(OP_GET, 0x0302, KIND_STR, ok, b"x")), None);
    assert_eq!(decode_str(HOSTNAME, &reply(9, HOSTNAME, KIND_STR, ok, b"x")), None);
    assert_eq!(decode_str(HOSTNAME, &reply(OP_GET, HOSTNAME, 1, ok, b"x")), None);
}

#[test]
fn an_error_reply_and_a_truncated_one_yield_nothing() {
    assert_eq!(decode_str(HOSTNAME, &reply(OP_GET, HOSTNAME, KIND_STR, 91, b"")), None);
    let mut short = reply(OP_GET, HOSTNAME, KIND_STR, E_OK, b"station");
    short.truncate(HDR_LEN + 3);
    assert_eq!(decode_str(HOSTNAME, &short), None);
    assert_eq!(decode_str(HOSTNAME, &[0u8; 4]), None);
}
