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

//! SPHINCS+ draws its three key seeds and each signature's randomiser from
//! the seeded RNG, and refuses when a draw fails.
//!
//! A whole signature is not tested. The kernel's signing shifts the 32-bit
//! leaf index by up to 54 bits (`sign.rs`, the hypertree loop): a debug build
//! panics there from the fourth layer, and a release build wraps to a wrong
//! index. That defect is older than this change and is reported, not fixed.

use crate::crypto::pqc::sphincs::{sphincs_keygen, sphincs_sign};
use crate::stand_in::{draws, fail_from, fail_only};

#[test]
fn a_key_pair_takes_three_fresh_seeds() {
    fail_from(None);
    let (a, b) = (sphincs_keygen().expect("drawn"), sphincs_keygen().expect("drawn"));
    assert_eq!(draws(), 12, "three 16-byte seeds, two words each, per key pair");
    assert_ne!(a.secret_key.sk_seed, b.secret_key.sk_seed);
    assert_ne!(a.secret_key.sk_prf, b.secret_key.sk_prf);
    assert_ne!(a.public_key.seed, b.public_key.seed);
    assert_eq!(a.public_key.seed, a.secret_key.pk_seed);
}

#[test]
fn any_one_failed_draw_refuses_the_key_pair() {
    // Each seed's draw failing alone, with every later draw working.
    for n in 1..=6 {
        fail_only(n);
        assert!(matches!(sphincs_keygen(), Err("no randomness")), "draw {n} alone");
    }
}

#[test]
fn each_failed_draw_refuses_the_key_pair() {
    for from in 1..=6 {
        fail_from(Some(from));
        assert!(matches!(sphincs_keygen(), Err("no randomness")), "draw {from}");
    }
}

#[test]
fn a_failed_draw_refuses_the_signature() {
    fail_from(None);
    let kp = sphincs_keygen().expect("drawn");
    fail_from(Some(1));
    assert!(matches!(sphincs_sign(&kp.secret_key, b"nsh"), Err("no randomness")));
}
