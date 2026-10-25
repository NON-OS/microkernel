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

//! PURCHASE's wire (op 19, plan/21, v1.4.42): names only from the store,
//! and the names with their orderId back from the wallet.

use nonos_nox_meter::mailbox::Mailbox;
use nonos_payment_proto::{parse_purchase, Names, Purchase, Ready, EBUSY, MAX_NAME, PURCHASE_TAKE};

fn names(listing: &[u8]) -> Names {
    Names { chain_id: 11_155_111, listing: listing.to_vec(), release: Vec::new() }
}

#[test]
fn the_wire_carries_names_and_nothing_else() {
    let n = names(b"linux.qwen-small");
    let post = n.post().unwrap();
    assert_eq!(parse_purchase(&post), Some(Purchase::Post(n.clone())));
    assert_eq!(Names::parse(&n.encode().unwrap()), Some(n));
    assert_eq!(parse_purchase(&[PURCHASE_TAKE]), Some(Purchase::Take));
    assert_eq!(parse_purchase(&[PURCHASE_TAKE, 0]), None, "TAKE carries nothing");
    assert_eq!(parse_purchase(&[4]), None);
    assert_eq!(names(b"").post(), None, "a listing is named");
    assert_eq!(names(&[b'a'; MAX_NAME + 1]).post(), None);
    assert!(names(&[b'a'; MAX_NAME]).post().is_some());
    let long = [&names(b"linux.a").post().unwrap()[..], &[0]].concat();
    assert_eq!(parse_purchase(&long), None, "a byte past the names");
}

#[test]
fn the_order_id_goes_back_to_the_store_once() {
    let r = Ready { names: names(b"linux.a"), order_id: [7; 32] };
    let post = r.post().unwrap();
    assert_eq!(parse_purchase(&post), Some(Purchase::Ready(r.clone())));
    assert_eq!(parse_purchase(&[3]), Some(Purchase::TakeReady));
    assert_eq!(parse_purchase(&post[..post.len() - 1]), None, "a short orderId");
    let mut m = Mailbox::default();
    m.ready.post(r.clone(), 0).unwrap();
    assert_eq!(m.ready.post(r.clone(), 1), Err(EBUSY));
    assert_eq!(m.names.take(1), None, "the slots are apart");
    assert_eq!(m.ready.take(2), Some(r));
    assert_eq!(m.ready.take(3), None);
}
