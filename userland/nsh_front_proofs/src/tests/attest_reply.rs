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

//! nsh.attest's answers are whole nsh.front frames: the header laid down in
//! front of every body, a document's or the entries' as much as an empty
//! refusal's, and read back by the same parse nsh.front's relay uses. A
//! frame whose header was never written reads as no frame at all, which is
//! what nsh.front said of every document before.

use crate::attest_reply::{err, frame};
use nsh_front_wire::{
    parse, HDR_LEN, OP_DOC, OP_ENTRIES, OP_HEALTH, ST_ATTEST_REFUSED, ST_OK, ST_REFUSED_CALLER,
};

const REPLY: &str = include_str!("../../../capsule_nsh_attest/src/reply.rs");

#[test]
fn a_document_answer_parses_with_its_op_status_and_body() {
    let doc: Vec<u8> = (0..4096u32).map(|i| (i * 7) as u8).collect();
    let f = frame(OP_DOC, ST_OK, &doc);
    assert_eq!(f.len(), HDR_LEN + doc.len());
    let h = parse(&f).expect("a whole frame");
    assert_eq!((h.op, h.status), (OP_DOC, ST_OK));
    assert_eq!(h.body, &doc[..]);
}

#[test]
fn an_entries_answer_of_any_length_parses() {
    for n in [1usize, 45, 45 * 1024, (1 << 20) - 4096] {
        let body = vec![0xA5u8; n];
        let f = frame(OP_ENTRIES, ST_OK, &body);
        let h = parse(&f).unwrap_or_else(|| panic!("{n} bytes"));
        assert_eq!((h.op, h.status, h.body.len()), (OP_ENTRIES, ST_OK, n));
    }
}

#[test]
fn a_refusal_is_the_header_alone() {
    for (op, st) in [(OP_DOC, ST_ATTEST_REFUSED), (OP_ENTRIES, ST_REFUSED_CALLER)] {
        let f = err(op, st);
        assert_eq!(f.len(), HDR_LEN);
        let h = parse(&f).expect("a whole frame");
        assert_eq!((h.op, h.status, h.body.len()), (op, st, 0));
    }
    let empty = frame(OP_HEALTH, ST_OK, &[]);
    let h = parse(&empty).expect("an empty OK");
    assert_eq!((h.op, h.status, h.body.len()), (OP_HEALTH, ST_OK, 0));
}

#[test]
fn a_frame_with_no_header_written_is_no_frame() {
    let mut f = frame(OP_DOC, ST_OK, &[1, 2, 3]);
    f[..HDR_LEN].fill(0);
    assert!(parse(&f).is_none());
}

#[test]
fn the_frame_is_sized_for_its_body_before_the_header_is_written() {
    let sized = REPLY.find("let mut out = vec![0u8; total];").expect("sized first");
    let header = REPLY.find("match write_header(&mut out, op, status, body.len())").expect("then");
    assert!(sized < header);
    assert!(!REPLY.contains("out.resize(HDR_LEN, 0);"));
}
