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

//! ORDER_* bodies, laid out by hand and read back.

use nonos_payment_proto::{parse_refund_reply, refund_reply, OrderMade, OrderNew, OrderStatus};
use nonos_payment_proto::{ORDER_MADE_LEN, ORDER_NEW_LEN, ORDER_STATUS_LEN};

fn new() -> OrderNew {
    OrderNew {
        chain_id: 11155111,
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [4; 20],
        amount: 1_000,
        chunk_root: [5; 32],
        content_len: 4096,
        cipher_root: [6; 32],
        key_commit: [7; 32],
        deadline: 1_900_000_000,
        refund_to: [8; 20],
    }
}

#[test]
fn order_new_is_its_fields_in_order() {
    let b = new().encode();
    assert_eq!(b.len(), ORDER_NEW_LEN);
    assert_eq!(&b[..8], &11155111u64.to_le_bytes());
    assert_eq!((&b[8..40], &b[104..124]), (&[1u8; 32][..], &[4u8; 20][..]));
    assert_eq!(&b[124..140], &1_000u128.to_le_bytes());
    assert_eq!(&b[b.len() - 20..], &[8u8; 20]);
    assert_eq!(OrderNew::parse(&b), Some(new()));
    assert_eq!(OrderNew::parse(&b[1..]), None);
    assert_eq!(OrderNew::parse(&[b.as_slice(), &[0]].concat()), None);
}

#[test]
fn the_replies_round_trip_exactly() {
    let made = OrderMade { order_id: [9; 32], deposit: [10; 20], terms: [[11; 32]; 14] };
    let b = made.encode();
    assert_eq!((b.len(), &b[32..52]), (ORDER_MADE_LEN, &[10u8; 20][..]));
    assert_eq!(OrderMade::parse(&b), Some(made));
    assert_eq!(OrderMade::parse(&b[..b.len() - 1]), None);
    let st = OrderStatus { state: 2, funded: 7, pending: 9, extra: 6, plain: true };
    assert_eq!((st.encode().len(), ORDER_STATUS_LEN), (50, 50), "v1.4.60's extra");
    assert_eq!(&st.encode()[33..49], &6u128.to_le_bytes(), "extra after pending");
    assert_eq!(OrderStatus::parse(&st.encode()), Some(st));
    let mut odd = st.encode();
    odd[49] = 2;
    assert_eq!(OrderStatus::parse(&odd), None, "plain is 0 or 1");
    assert_eq!(OrderStatus::parse(&st.encode()[..33]), None, "the 33-byte form is gone");
    let old = [&st.encode()[..33], &[1u8][..]].concat();
    assert_eq!(OrderStatus::parse(&old), None, "the 34-byte form without extra is refused");
    let r = refund_reply(&[1; 20], &[2; 32]);
    assert_eq!(parse_refund_reply(&r), Some(([1; 20], [2; 32])));
    assert_eq!(parse_refund_reply(&r[..51]), None);
}
