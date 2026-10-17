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

//! An order takes the kept offer its terms commit to, and
//! ORDER_DELIVERED opens that offer: the release's bytes under the right K,
//! the dispute for a bad chunk, and nothing under another K or offer.

use nonos_nox_chunks::DISPUTE;
use nonos_nox_meter::orders::{deliver, offer_for, order_takes, Opened};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::OrderNew;

use super::deliver_fixture::{kept, order};
use super::host::Host;
use super::seller::{content, K};

#[test]
fn an_order_takes_only_the_offer_its_terms_name() {
    let k = kept();
    let o = order(&k);
    assert!(order_takes(&o, &k));
    let mut other = o;
    other.terms[11] = [0xee; 32];
    assert!(!order_takes(&other, &k), "another keyCommit");
    let n = OrderNew {
        chain_id: k.chain_id,
        listing_id: k.listing_key,
        release_id: k.release_key,
        bond_id: [3; 32],
        payee: [4; 20],
        amount: 1_000,
        chunk_root: k.chunk_root,
        content_len: k.content_len,
        cipher_root: k.offer.cipher_root,
        key_commit: k.offer.key_commit,
        deadline: 9,
        refund_to: [8; 20],
        refund_salt: [9; 32],
    };
    let list = vec![(2, k.clone())];
    assert_eq!(offer_for(&n, &list).map(|(s, _)| *s), Some(2));
    assert!(offer_for(&OrderNew { release_id: [3; 32], ..n }, &list).is_none(), "another release");
    assert!(offer_for(&OrderNew { content_len: 1, ..n }, &list).is_none(), "another length");
}

#[test]
fn a_delivery_opens_to_the_release_or_names_the_bad_chunk() {
    let k = kept();
    let o = order(&k);
    assert_eq!(deliver(&Host, &o, &k, &K), Ok(Opened::Plain(content())));
    assert_eq!(deliver(&Host, &o, &k, &[0x43; 32]), Err(MeterError::Untrusted), "another K");
    let mut lying = k.clone();
    lying.offer.ciphertext[2 * 1024 + 5] ^= 1;
    let Ok(Opened::Bad(2, call)) = deliver(&Host, &o, &lying, &K) else {
        panic!("chunk 2 not named")
    };
    assert_eq!(call[..4], DISPUTE.to_be_bytes());
    let mut not_its = o;
    not_its.terms[10] = [0xee; 32];
    assert_eq!(deliver(&Host, &not_its, &k, &K), Err(MeterError::Mismatch));
}
