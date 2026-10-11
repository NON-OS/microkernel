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
use nonos_nox_chunks::{check_offer, chunk_root, key_commit, OfferWrong};
use seller::{content, offer, KEY};

#[test]
fn an_honest_offer_passes_and_every_change_is_caught() {
    let c = content();
    let (root, len) = (chunk_root(&K, &c).unwrap(), c.len() as u64);
    let (h, ct, cr) = offer(&c, None);
    assert_eq!(check_offer(&K, &root, len, &h, &ct, &cr), Ok(()));
    assert_eq!(check_offer(&K, &root, len + 1024, &h, &ct, &cr), Err(OfferWrong::HashCount));
    assert_eq!(check_offer(&K, &root, 0, &[], &[], &cr), Err(OfferWrong::HashCount));
    let mut bad_h = h.clone();
    bad_h[1][0] ^= 1;
    assert_eq!(check_offer(&K, &root, len, &bad_h, &ct, &cr), Err(OfferWrong::ChunkRoot));
    assert_eq!(check_offer(&K, &root, len, &h, &ct[..2048], &cr), Err(OfferWrong::CipherLength));
    assert_eq!(check_offer(&K, &root, len, &h, &ct[1..], &cr), Err(OfferWrong::CipherLength));
    let mut bad_ct = ct.clone();
    bad_ct[5] ^= 1;
    assert_eq!(check_offer(&K, &root, len, &h, &bad_ct, &cr), Err(OfferWrong::CipherRoot));
    assert_eq!(key_commit(&K, &KEY), kec(&[&KEY]));
}

#[test]
fn a_lie_in_one_chunk_still_passes_the_offer() {
    let c = content();
    let (h, ct, cr) = offer(&c, Some(1));
    let root = chunk_root(&K, &c).unwrap();
    assert_eq!(
        check_offer(&K, &root, c.len() as u64, &h, &ct, &cr),
        Ok(()),
        "only the key shows it"
    );
}
