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
#[path = "common/seller.rs"]
mod seller;

use k::{kec, K};
use nonos_nox_chunks::{chunk_root, dispute, leaf, open_delivery, verify, xor_chunk, Delivered};
use nonos_nox_chunks::{CIPHER_TREE, PLAIN_TREE};
use seller::{content, offer, KEY};

const COMMIT: fn() -> [u8; 32] = || kec(&[&KEY]);

#[test]
fn the_right_key_opens_an_honest_delivery() {
    let c = content();
    let (h, ct, _) = offer(&c, None);
    assert_eq!(open_delivery(&K, &KEY, &COMMIT(), &h, &ct, c.len() as u64), Ok(c.clone()));
    let other = [0x43; 32];
    assert_eq!(open_delivery(&K, &other, &COMMIT(), &h, &ct, 3000), Err(Delivered::WrongKey));
    assert_eq!(
        open_delivery(&K, &KEY, &COMMIT(), &h, &ct[..2048], 3000),
        Err(Delivered::NotTheOffer)
    );
    assert_eq!(open_delivery(&K, &KEY, &COMMIT(), &h, &ct, 3073), Err(Delivered::NotTheOffer));
}

#[test]
fn a_lying_chunk_is_found_and_its_dispute_convicts() {
    let c = content();
    let (h, ct, cipher_root) = offer(&c, Some(1));
    assert_eq!(open_delivery(&K, &KEY, &COMMIT(), &h, &ct, 3000), Err(Delivered::BadChunk(1)));
    let d = dispute(&K, 1, &h, &ct).unwrap();
    assert_eq!((d.index, d.plain_hash, d.cipher_chunk.len()), (1, h[1], 1024));
    let c_leaf = leaf(&K, CIPHER_TREE, 1, &kec(&[&d.cipher_chunk]));
    assert!(verify(&K, &c_leaf, &d.cipher_proof, &cipher_root), "c_1 is in cipherRoot");
    let p_leaf = leaf(&K, PLAIN_TREE, 1, &d.plain_hash);
    assert!(
        verify(&K, &p_leaf, &d.leaf_proof, &chunk_root(&K, &c).unwrap()),
        "its hash is in chunk_root"
    );
    let opened = xor_chunk(&K, &KEY, 1, d.cipher_chunk.as_slice().try_into().unwrap());
    assert_ne!(kec(&[&opened]), d.plain_hash, "what the contract sees");
    assert_eq!(dispute(&K, 3, &h, &ct), None);
    assert_eq!(dispute(&K, 0, &h, &ct[1..]), None);
}
