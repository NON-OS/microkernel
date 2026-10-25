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

use nonos_nox_meter::orders::ORDER_LEN;
use nonos_nox_meter::orders::{decode_orders, encode_orders, Order, MAX_ORDERS, ORDERS_LEN};

fn order(i: u8) -> Order {
    Order {
        chain_id: 11155111,
        order_id: [i; 32],
        deposit: [i ^ 0x80; 20],
        terms: [[i; 32]; 14],
        refund_to: [0xaa; 20],
        refund_salt: [0xbb; 32],
        refund_taken: i.is_multiple_of(2),
        plain: i.is_multiple_of(3),
    }
}

#[test]
fn the_table_reads_back_at_one_length() {
    let some: Vec<Order> = (1..=3).map(order).collect();
    let b = encode_orders(&some).unwrap();
    assert_eq!(b.len(), ORDERS_LEN);
    assert_eq!(decode_orders(&b), Some(some));
    let full: Vec<Order> = (1..=MAX_ORDERS as u8).map(order).collect();
    assert_eq!(decode_orders(&encode_orders(&full).unwrap()), Some(full.clone()));
    assert_eq!(encode_orders(&[full.as_slice(), &[order(99)]].concat()), None);
    assert_eq!(decode_orders(&encode_orders(&[]).unwrap()), Some(vec![]));
}

#[test]
fn nothing_else_reads_as_orders() {
    let b = encode_orders(&[order(1)]).unwrap();
    let no = |b: &[u8], why: &str| assert_eq!(decode_orders(b), None, "{why}");
    no(&b[..ORDERS_LEN - 1], "short");
    let mut magic = b.clone();
    magic[0] ^= 1;
    no(&magic, "another magic");
    let mut dirty = b.clone();
    dirty[ORDERS_LEN - 1] = 1;
    no(&dirty, "a byte in an unused slot");
    let mut many = b.clone();
    many[6] = 2;
    no(&many, "a count past the slots that hold orders");
    let mut zero_chain = b.clone();
    zero_chain[8..16].fill(0);
    no(&zero_chain, "a used slot with chain 0");
    let mut flag = b.clone();
    flag[8 + ORDER_LEN - 1] = 4;
    no(&flag, "a flag past bit 1 (refund taken, plain)");
    let mut plain = b.clone();
    plain[8 + ORDER_LEN - 1] = 2;
    let read = decode_orders(&plain).unwrap();
    assert!(read[0].plain && !read[0].refund_taken, "v1.4.42's plain bit");
}
