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

//! The offer inside a sealed answer reads back exactly, and the Equi-X
//! challenge is listing_key | buyer_pk.

#[path = "common/k.rs"]
mod k;
#[path = "common/seller.rs"]
mod seller;

use k::{kec, K};
use nonos_nox_chunks::{equix_challenge, key_commit, Offer};
use seller::{content, offer, KEY};

fn honest() -> Offer {
    let (hashes, ciphertext, cipher_root) = offer(&content(), None);
    Offer { cipher_root, key_commit: key_commit(&K, &KEY), hashes, ciphertext }
}

#[test]
fn an_offer_body_reads_back_exactly() {
    let o = honest();
    let b = o.encode();
    assert_eq!(b.len(), 68 + 32 * o.hashes.len() + o.ciphertext.len());
    assert_eq!(Offer::parse(&b), Some(o.clone()));
    assert_eq!(Offer::parse(&b[..68 + 31]), None, "short of its own count");
    let mut huge = b.clone();
    huge[64..68].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(Offer::parse(&huge), None);
    assert_eq!(o.key_commit, kec(&[&KEY]), "keyCommit is keccak256(abi.encode(K))");
    assert_eq!(equix_challenge(&[1; 32], &[2; 32])[31..33], [1, 2]);
}
