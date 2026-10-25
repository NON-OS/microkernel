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

use nonos_settle_proto::{admits_flags, async_answer, parse_header, reply, request, request_async};
use nonos_settle_proto::{ReplyError, FLAG_ASYNC, OP_GAS};

const PAYMENT: u32 = 41;
const SETTLE: u32 = 42;
const MARKET: u32 = 43;

fn owner(name: &[u8]) -> Option<u32> {
    match name {
        b"payment" => Some(PAYMENT),
        b"market.index" => Some(MARKET),
        b"app.store" => Some(99),
        _ => None,
    }
}

#[test]
fn an_async_request_is_a_plain_one_with_bit_0() {
    let a = request_async(OP_GAS, 7, &[1, 2]).unwrap();
    let p = request(OP_GAS, 7, &[1, 2]).unwrap();
    let (h, body) = parse_header(&a).unwrap();
    assert_eq!((h.op, h.flags, h.request_id, body), (OP_GAS, FLAG_ASYNC, 7, &[1u8, 2][..]));
    assert_eq!(a[..8], p[..8]);
    assert_eq!(a[10..], p[10..], "only the flags differ");
}

#[test]
fn only_payment_and_the_market_may_ask_async() {
    assert!(admits_flags(0, OP_GAS, 99, owner), "a plain call is the op table's business");
    assert!(admits_flags(FLAG_ASYNC, OP_GAS, PAYMENT, owner));
    assert!(admits_flags(FLAG_ASYNC, OP_GAS, MARKET, owner), "v1.4.23: its bond cache");
    assert!(!admits_flags(FLAG_ASYNC, OP_GAS, 99, owner));
    assert!(!admits_flags(FLAG_ASYNC, OP_GAS, 0, |_| Some(0)));
    assert!(!admits_flags(FLAG_ASYNC | 2, OP_GAS, 99, owner));
}

#[test]
fn an_answer_is_believed_only_from_settle_for_its_own_id() {
    let rx = reply(OP_GAS, FLAG_ASYNC, 7, 0, &[9]).unwrap();
    assert_eq!(async_answer(&rx, SETTLE, SETTLE, OP_GAS, 7), Ok(&[9u8][..]));
    assert_eq!(async_answer(&rx, 99, SETTLE, OP_GAS, 7), Err(ReplyError::Foreign));
    assert_eq!(async_answer(&rx, 0, 0, OP_GAS, 7), Err(ReplyError::Foreign));
    assert_eq!(async_answer(&rx, SETTLE, SETTLE, OP_GAS, 8), Err(ReplyError::Stale));
    let failed = reply(OP_GAS, FLAG_ASYNC, 7, -5, &[]).unwrap();
    assert_eq!(async_answer(&failed, SETTLE, SETTLE, OP_GAS, 7), Err(ReplyError::Status(-5)));
    assert_eq!(async_answer(b"NPAY....", SETTLE, SETTLE, OP_GAS, 7), Err(ReplyError::Foreign));
}
