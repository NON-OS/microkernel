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

#[path = "common/k.rs"]
mod k;
#[path = "common/word.rs"]
mod word;

use k::{kec, K};
use nonos_nox_chunks::{order_id, refund_commit, OrderTerms};
use nonos_nox_config::{ORDER_TAG, REFUND_TAG};
use word::word;

fn terms() -> OrderTerms {
    OrderTerms {
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [4; 20],
        token: [5; 20],
        amount: 6_000_000_000_000_000_000,
        delivery_deadline: 7,
        dispute_window: 8,
        chunk_root: [9; 32],
        content_len: 10,
        cipher_root: [11; 32],
        key_commit: [12; 32],
        refund_commit: [13; 32],
        salt: [14; 32],
    }
}

#[test]
fn ids_are_abi_encode_under_their_tags() {
    let t = terms();
    let mut parts: Vec<[u8; 32]> =
        vec![ORDER_TAG, word(&11155111u64.to_be_bytes()), word(&[0xEE; 20])];
    parts.extend(t.words());
    assert_eq!(order_id(&K, 11155111, &[0xEE; 20], &t), kec(&[&parts.concat()]));
    assert_ne!(order_id(&K, 1, &[0xEE; 20], &t), order_id(&K, 11155111, &[0xEE; 20], &t));
    let want = kec(&[&REFUND_TAG, &word(&[0xAB; 20]), &[0xCD; 32]]);
    assert_eq!(refund_commit(&K, &[0xAB; 20], &[0xCD; 32]), want);
}
