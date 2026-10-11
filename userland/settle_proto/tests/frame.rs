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

use nonos_settle_proto::*;

#[test]
fn the_header_is_market_protos_under_nstl_and_replies_are_checked() {
    let f = request(OP_GAS, 9, &[1, 2]);
    assert_eq!(&f[..4], b"NSTL");
    assert_eq!(&f[4..20], &[1, 0, 5, 0, 0, 0, 0, 0, 9, 0, 0, 0, 2, 0, 0, 0]);
    let (h, body) = parse_header(&f).unwrap();
    assert_eq!((h.op, h.request_id, body), (OP_GAS, 9, &[1u8, 2][..]));
    for n in 0..f.len() {
        assert!(parse_header(&f[..n]).is_none(), "{n} bytes");
    }
    let r = reply(OP_OPENED, 0, 4, EAGAIN, &[]);
    assert_eq!(reply_body(&r, OP_OPENED, 4), Err(ReplyError::Status(-11)));
    assert_eq!(reply_body(&r, OP_OPENED, 5), Err(ReplyError::Stale));
    let ok = reply(OP_BROADCAST, 0, 1, 0, &[7; 32]);
    assert_eq!(reply_body(&ok, OP_BROADCAST, 1), Ok(&[7u8; 32][..]));
}

#[test]
fn ops_and_the_service_are_pinned() {
    assert_eq!(SERVICE, b"nox_settle");
    let ops = [OP_HEALTHCHECK, OP_BROADCAST, OP_READ_NONCE, OP_CHANNEL_STATE, OP_GAS, OP_OPENED];
    assert_eq!(ops, [1, 2, 3, 4, 5, 6]);
}
