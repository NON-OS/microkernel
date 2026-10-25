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

//! ORDER_NEW's refundSalt, the wallet's own draw: the last 32 bytes, the
//! body exact at 304, and never shown by Debug.

use nonos_payment_proto::{OrderNew, ORDER_NEW_LEN};

fn salted(salt: [u8; 32]) -> OrderNew {
    OrderNew {
        chain_id: 1,
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [4; 20],
        amount: 5,
        chunk_root: [6; 32],
        content_len: 7,
        cipher_root: [8; 32],
        key_commit: [9; 32],
        deadline: 10,
        refund_to: [0x11; 20],
        refund_salt: salt,
    }
}

#[test]
fn the_refund_salt_rides_last_and_round_trips() {
    let n = salted([0xc3; 32]);
    let b = n.encode();
    assert_eq!((b.len(), ORDER_NEW_LEN), (304, 304), "272 and refundSalt[32]");
    assert_eq!((&b[252..272], &b[272..]), (&[0x11u8; 20][..], &[0xc3u8; 32][..]));
    assert_eq!(OrderNew::parse(&b), Some(n));
    let mut flipped = b.clone();
    flipped[303] ^= 1;
    assert_eq!(OrderNew::parse(&flipped).map(|o| o.refund_salt[31]), Some(0xc2), "read as sent");
}

#[test]
fn a_body_without_the_salt_or_longer_is_refused() {
    let b = salted([0xc3; 32]).encode();
    assert_eq!(OrderNew::parse(&b[..272]), None, "the old body, no refundSalt");
    assert_eq!(OrderNew::parse(&b[..303]), None);
    assert_eq!(OrderNew::parse(&[b.as_slice(), &[0]].concat()), None);
    assert_eq!(OrderNew::parse(&[]), None);
}

#[test]
fn debug_never_shows_the_salt() {
    let shown = format!("{:?}", salted([0xc3; 32]));
    assert!(shown.contains("refund_salt: \"redacted\""), "{shown}");
    assert!(!shown.contains("195"), "0xc3 is 195: {shown}");
    assert!(shown.contains("refund_to: [17,"), "{shown}");
}
