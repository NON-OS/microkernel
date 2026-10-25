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

//! ORDER_OFFER's request and result: byte for byte, with every cut,
//! trailing byte and over-long url refused.

use nonos_payment_proto::{
    OfferAsk, OfferMade, MAX_URL, OFFER_MADE_LEN, OP_ORDER_OFFER, OP_ORDER_PLAINTEXT,
};

fn ask() -> OfferAsk {
    OfferAsk {
        chain_id: 11_155_111,
        listing_key: [1; 32],
        release_key: [2; 32],
        chunk_root: [3; 32],
        content_len: 5000,
        url: b"https://agent.example/e".to_vec(),
    }
}

#[test]
fn an_offer_request_reads_back_exactly() {
    let b = ask().encode().unwrap();
    assert_eq!(b.len(), 8 + 96 + 8 + 2 + 23);
    assert_eq!(
        (&b[..8], b[8], b[72], &b[104..112]),
        (&11_155_111u64.to_le_bytes()[..], 1, 3, &5000u64.to_le_bytes()[..])
    );
    assert_eq!(OfferAsk::parse(&b), Some(ask()));
    for n in 0..b.len() {
        assert_eq!(OfferAsk::parse(&b[..n]), None, "cut at {n}");
    }
    assert_eq!(OfferAsk::parse(&[b.as_slice(), &[0]].concat()), None);
    let long = OfferAsk { url: vec![b'a'; MAX_URL + 1], ..ask() };
    assert_eq!(long.encode(), None);
    assert_eq!((OP_ORDER_OFFER, OP_ORDER_PLAINTEXT), (17, 18));
}

#[test]
fn an_offer_result_is_its_three_words() {
    let m = OfferMade { offer_id: [7; 32], cipher_root: [8; 32], key_commit: [9; 32] };
    let b = m.encode();
    assert_eq!((b.len(), b[0], b[32], b[64]), (OFFER_MADE_LEN, 7, 8, 9));
    assert_eq!(OfferMade::parse(&b), Some(m));
    assert_eq!(OfferMade::parse(&b[..95]), None);
}
