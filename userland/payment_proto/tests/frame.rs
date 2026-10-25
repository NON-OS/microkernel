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

//! The header and the reply checks, against bytes laid out here by hand.

use nonos_payment_proto::*;

#[test]
fn a_request_is_market_protos_header_under_npay() {
    let f = request(OP_PAY, 7, &[1, 2, 3]).unwrap();
    assert_eq!(&f[..4], b"NPAY");
    assert_eq!(&f[4..20], &[1, 0, 2, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3, 0, 0, 0]);
    let (h, body) = parse_header(&f).unwrap();
    assert_eq!((h.op, h.request_id, body), (OP_PAY, 7, &[1u8, 2, 3][..]));
}

#[test]
fn a_frame_that_is_cut_long_foreign_or_reserved_is_refused() {
    let f = request(OP_DRAIN, 1, &[9]).unwrap();
    for n in 0..f.len() {
        assert!(parse_header(&f[..n]).is_none(), "{n} bytes");
    }
    let mut long = f.clone();
    long.push(0);
    assert!(parse_header(&long).is_none());
    for at in [0, 4, 10] {
        let mut bad = f.clone();
        bad[at] ^= 1;
        assert!(parse_header(&bad).is_none(), "byte {at}");
    }
}

#[test]
fn a_reply_is_believed_only_for_its_own_call_and_status() {
    let r = reply(OP_PAY, 0, 5, OK, &[4, 4]).unwrap();
    assert_eq!(reply_body(&r, OP_PAY, 5), Ok(&[4u8, 4][..]));
    assert_eq!(reply_body(&r, OP_PAY, 6), Err(ReplyError::Stale));
    assert_eq!(reply_body(&r, OP_DRAIN, 5), Err(ReplyError::Stale));
    let no = reply(OP_PAY, 0, 5, ENOPERSIST, &[]).unwrap();
    assert_eq!(reply_body(&no, OP_PAY, 5), Err(ReplyError::Status(-1001)));
    assert_eq!(reply_body(&r[..r.len() - 1], OP_PAY, 5), Err(ReplyError::Foreign));
}

#[test]
fn ops_and_the_meters_own_statuses_are_pinned() {
    let ops = [OP_HEALTHCHECK, OP_PAY, OP_DRAIN, OP_LIST_TOKENS, OP_ACCOUNT, OP_SET_BUDGET];
    assert_eq!(ops, [1, 2, 3, 4, 5, 6]);
    let more =
        [OP_ENTITLE, OP_ORDER_NEW, OP_ORDER_STATUS, OP_ORDER_DELIVERED, OP_ORDER_REFUND, OP_OPEN];
    assert_eq!(more, [7, 8, 9, 10, 11, 12]);
    assert_eq!((ENOPERSIST, EFROZEN, EGAS), (-1001, -1002, -1003));
    assert_eq!(SERVICE, b"payment");
}

#[test]
fn a_status_reply_is_a_reply_with_no_body_and_always_frames() {
    let s = status_reply(OP_PAY, 0, 4, EAGAIN);
    assert_eq!(Some(s.clone()), reply(OP_PAY, 0, 4, EAGAIN, &[]));
    assert_eq!(reply_body(&s, OP_PAY, 4), Err(ReplyError::Status(EAGAIN)));
}
