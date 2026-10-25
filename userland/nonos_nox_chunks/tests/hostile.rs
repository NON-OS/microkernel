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

//! Hostile input from a seller's agent or a chain read (the owner's rule 8):
//! random sealed answers, offers, pins, proof-of-work fields, terms words,
//! proofs and deliveries, with content lengths up to u64::MAX. Nothing may
//! panic, and nothing random passes a check meant for the honest seller.

#[path = "common/k.rs"]
mod k;
#[path = "common/x.rs"]
mod x;

use k::K;
use nonos_nox_chunks::*;
use x::D;

fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect()
}

fn words(b: &[u8]) -> Vec<[u8; 32]> {
    b.chunks_exact(32).map(|w| w.try_into().unwrap()).collect()
}

const LENS: [u64; 7] = [0, 1, 1024, 1025, 70_000, u64::MAX / 2, u64::MAX];

#[test]
fn nothing_random_parses_opens_or_checks() {
    let (r, key) = ([1u8; 32], [2u8; 32]);
    assert_eq!(nonos_marketplace_abi::Keccak::keccak256(&K, b"x"), k::kec(&[b"x"]));
    for n in (0..200).chain([1088, 2176, 4096]) {
        let b = bytes(n as u64 + 11, n);
        let hs = words(&b);
        let _ = (PowField::parse(&b), EscrowPin::parse(&b), pow_check(&r, &r, &b, 8));
        for len in LENS {
            let _ = Offer::parse(&b, len);
            assert!(open_answer((&K, &D), &key, ((&r, &r), len), &b).is_err(), "{n} {len}");
            assert!(open_delivery(&K, &key, &r, &hs, &b, len).is_err());
            assert!(check_offer(&K, &r, len, &hs, &b, &r).is_err());
            let _ = (chunk_count(len), offer_len(len), dispute(&K, len, &hs, &b));
        }
        assert!(!verify(&K, &r, &hs, &r) || hs.is_empty());
        if let Ok(w) = <[[u8; 32]; TERMS_WORDS]>::try_from(hs.get(..TERMS_WORDS).unwrap_or(&[])) {
            if let Some(t) = OrderTerms::from_words(&w) {
                assert_eq!(t.words(), w, "terms that read encode back");
            }
        }
    }
}
