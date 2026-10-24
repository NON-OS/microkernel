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

//! The kernel's ML-DSA-65 key pair and signature, over the pinned C, with
//! the seeded RNG made to fail: nothing made from a failed draw is handed
//! back, the C hook itself reports the failure, and a later draw that
//! works is used as normal.

use crate::crypto::pqc::ml_dsa_65::types::MlDsa65Error;
use crate::crypto::pqc::ml_dsa_65::{ml_dsa_65_keypair, ml_dsa_65_sign, ml_dsa_65_verify};
use crate::stand_in::{fail_draws, turn};

extern "C" {
    fn randombytes(out: *mut u8, n: usize) -> i32;
}

#[test]
fn with_a_working_rng_a_key_pair_signs() {
    let _t = turn();
    fail_draws(None);
    let kp = ml_dsa_65_keypair().expect("key pair");
    let sig = ml_dsa_65_sign(&kp.secret_key, b"nonos").expect("signature");
    assert!(ml_dsa_65_verify(&kp.public_key, b"nonos", &sig), "it signs the message given");
    assert!(!ml_dsa_65_verify(&kp.public_key, b"nonot", &sig));
}

#[test]
fn a_failed_draw_refuses_the_key_pair() {
    let _t = turn();
    fail_draws(Some((1, false)));
    assert!(matches!(ml_dsa_65_keypair(), Err(MlDsa65Error::RandomnessFailed)));
    assert!(ml_dsa_65_keypair().is_ok(), "the next draw works, and is used");
}

#[test]
fn a_failed_draw_refuses_the_signature_and_only_that() {
    let _t = turn();
    fail_draws(Some((2, false)));
    let kp = ml_dsa_65_keypair().expect("draw 1 works");
    assert!(matches!(ml_dsa_65_sign(&kp.secret_key, b"m"), Err(MlDsa65Error::RandomnessFailed)));
    assert!(ml_dsa_65_sign(&kp.secret_key, b"m").is_ok());
}

#[test]
fn the_c_hook_returns_failure_and_hands_over_zeros() {
    let _t = turn();
    fail_draws(Some((1, false)));
    let mut buf = [0xAAu8; 48];
    // SAFETY: eK@nonos.systems - `buf` is 48 writable bytes.
    assert_eq!(unsafe { randombytes(buf.as_mut_ptr(), buf.len()) }, -1);
    assert_eq!(buf, [0; 48]);
    // SAFETY: as above.
    assert_eq!(unsafe { randombytes(buf.as_mut_ptr(), buf.len()) }, 0);
    assert_ne!(buf, [0; 48]);
}
