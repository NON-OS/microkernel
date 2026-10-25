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

//! The crate's own refusals. A source that cannot draw fails the whole
//! call: the C does not check its draws, and would otherwise make keys or
//! signatures from zeros. A changed byte anywhere fails verification.

use nonos_mldsa65::{keypair, sign, verify, Error};

pub fn counter(seed: u8) -> impl FnMut(&mut [u8]) -> bool {
    let mut n = seed;
    move |b: &mut [u8]| {
        b.iter_mut().for_each(|x| {
            n = n.wrapping_mul(29).wrapping_add(11);
            *x = n;
        });
        true
    }
}

#[test]
fn a_failed_draw_fails_the_key_pair_and_the_signature() {
    assert!(matches!(keypair(&mut |_: &mut [u8]| false), Err(Error::Randomness)));
    let (_, sk) = keypair(&mut counter(1)).unwrap();
    assert_eq!(sign(&sk, b"m", &mut |_: &mut [u8]| false), Err(Error::Randomness));
}

#[test]
fn the_same_randomness_gives_the_same_key_and_other_randomness_another() {
    let (a, _) = keypair(&mut counter(1)).unwrap();
    let (b, _) = keypair(&mut counter(1)).unwrap();
    let (c, _) = keypair(&mut counter(2)).unwrap();
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn any_changed_byte_fails_verification() {
    let (pk, sk) = keypair(&mut counter(3)).unwrap();
    let sig = sign(&sk, b"nsh/1 host-sig", &mut counter(4)).unwrap();
    assert!(verify(&pk, b"nsh/1 host-sig", &sig));
    assert!(!verify(&pk, b"nsh/1 host-sih", &sig));
    for i in [0, 1000, 3308] {
        let mut bad = sig;
        bad[i] ^= 1;
        assert!(!verify(&pk, b"nsh/1 host-sig", &bad), "signature byte {i}");
    }
    let mut other = pk;
    other[17] ^= 0x80;
    assert!(!verify(&other, b"nsh/1 host-sig", &sig));
}
