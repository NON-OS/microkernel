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

//! ORDER_DELIVERED's request and result (plan/29), exactly.

use nonos_payment_proto::{DeliveredAsk, DeliveredMade, DELIVERED_ASK_LEN};

#[test]
fn a_delivery_request_is_the_order_and_k() {
    let a = DeliveredAsk { order_id: [1; 32], key: [2; 32] };
    let b = a.encode();
    assert_eq!((b.len(), b[0], b[32]), (DELIVERED_ASK_LEN, 1, 2));
    assert_eq!(DeliveredAsk::parse(&b), Some(a));
    assert_eq!(DeliveredAsk::parse(&b[..63]), None);
    assert_eq!(DeliveredAsk::parse(&[b.as_slice(), &[0]].concat()), None);
}

#[test]
fn a_delivery_result_is_ok_with_its_length_or_the_dispute() {
    let ok = DeliveredMade::Ok { content_len: 70_000 };
    let b = ok.encode();
    assert_eq!((b.len(), b[0]), (9, 0));
    assert_eq!(DeliveredMade::parse(&b), Some(ok));
    assert_eq!(
        DeliveredMade::parse(&[b.as_slice(), &[0]].concat()),
        None,
        "OK carries nothing more"
    );
    let bad = DeliveredMade::BadChunk { index: 3, calldata: vec![0xab; 100] };
    let b = bad.encode();
    assert_eq!((b[0], &b[1..9], b.len()), (1, &3u64.to_le_bytes()[..], 109));
    assert_eq!(DeliveredMade::parse(&b), Some(bad));
    assert_eq!(DeliveredMade::parse(&b[..9]), None, "a bad chunk carries its calldata");
    assert_eq!(DeliveredMade::parse(&[2, 0, 0, 0, 0, 0, 0, 0, 0]), None);
}

#[test]
fn the_content_key_never_shows_in_debug() {
    let shown = format!("{:?}", DeliveredAsk { order_id: [1; 32], key: [0xee; 32] });
    assert!(!shown.contains("238") && shown.contains("key: \"redacted\""), "{shown}");
}

#[test]
fn the_content_key_is_wiped_as_the_ask_drops() {
    let mut a = DeliveredAsk { order_id: [1; 32], key: [0xee; 32] };
    a.wipe();
    assert_eq!((a.key, a.order_id), ([0; 32], [1; 32]));
}
