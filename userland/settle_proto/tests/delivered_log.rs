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

//! The escrow's Delivered log (plan/24): its topic pinned, and the orderId
//! and key read whole or not at all.

use nonos_settle_proto::{admits, delivered, DeliveredKeys, DELIVERED, DELIVERED_TOPIC};
use nonos_settle_proto::{MAX_KEYS, OP_DELIVERED};
use sha3::{Digest, Keccak256};

#[test]
fn a_delivered_log_gives_its_order_and_key() {
    assert_eq!(<[u8; 32]>::from(Keccak256::digest(DELIVERED)), DELIVERED_TOPIC);
    assert_eq!(delivered(&[DELIVERED_TOPIC, [1; 32]], &[2; 32]), Some(([1; 32], [2; 32])));
}

#[test]
fn anything_else_is_no_event() {
    let t = [DELIVERED_TOPIC, [1; 32]];
    assert_eq!(delivered(&[[9; 32], [1; 32]], &[2; 32]), None, "another event");
    assert_eq!(delivered(&t[..1], &[2; 32]), None, "no orderId topic");
    assert_eq!(delivered(&[DELIVERED_TOPIC, [1; 32], [3; 32]], &[2; 32]), None, "a topic too many");
    assert_eq!(delivered(&t, &[2; 31]), None, "a short key");
    assert_eq!(delivered(&t, &[2; 64]), None, "a word too many");
}

#[test]
fn the_cache_finds_what_it_read_and_gives_up_past_its_bound() {
    let mut k = DeliveredKeys::new(11_155_111);
    k.absorb(vec![([1; 32], [2; 32])], 100);
    assert_eq!((k.find(&[1; 32]), k.find(&[3; 32]), k.scanned), (Some([2; 32]), None, 100));
    let many = (0..MAX_KEYS).map(|i| ([i as u8; 32], [0; 32])).collect();
    k.absorb(many, 200);
    assert!(k.full && k.keys.is_empty(), "past MAX_KEYS it rescans per order");
    assert_eq!(k.scanned, 100, "a full cache claims no blocks");
}

#[test]
fn the_meter_alone_asks_for_a_key() {
    let owner = |n: &[u8]| match n {
        b"payment" => Some(5),
        b"app.nonos_wallet" => Some(10),
        b"installer" => Some(30),
        b"app.store" => Some(31),
        _ => None,
    };
    assert!(admits(OP_DELIVERED, 5, owner));
    for (pid, who) in [(10, "the wallet"), (30, "the installer"), (31, "the store")] {
        assert!(!admits(OP_DELIVERED, pid, owner), "not {who}: K never passes them");
    }
}
