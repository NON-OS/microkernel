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

#[path = "common/word.rs"]
mod word;

use nonos_nox_chunks::{OrderTerms, TERMS_WORDS};
use nonos_settle_proto::ORDER_TERMS_TUPLE;
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
fn the_terms_are_the_tuple_fund_takes() {
    let types: Vec<&str> = ORDER_TERMS_TUPLE.trim_matches(['(', ')']).split(',').collect();
    assert_eq!(types.len(), TERMS_WORDS);
    for (i, ty) in types.iter().enumerate() {
        let want = match i {
            3 | 4 => "address",
            5 => "uint256",
            6 | 7 | 9 => "uint64",
            _ => "bytes32",
        };
        assert_eq!(*ty, want, "field {i}");
    }
    let t = terms();
    let n = |v: u64| word(&v.to_be_bytes());
    let want = [
        t.listing_id,
        t.release_id,
        t.bond_id,
        word(&t.payee),
        word(&t.token),
        word(&t.amount.to_be_bytes()),
        n(7),
        n(8),
        t.chunk_root,
        n(10),
        t.cipher_root,
        t.key_commit,
        t.refund_commit,
        t.salt,
    ];
    assert_eq!(t.words(), want);
}
