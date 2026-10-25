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

//! A failed draw from the system is refused, never turned into a key or a
//! signature; the all-zero mode of a reproducible signature never reaches a
//! key pair, even with both running at once.

use super::test_hooks::Forced;
use super::{ffi, guard::PQ, MlDsa65};
use crate::algs::traits::Signer;

/// The key pair the zero mode would make, for comparison.
fn zero_key() -> Vec<u8> {
    let _turn = PQ.lock().unwrap_or_else(|p| p.into_inner());
    let (mut pk, mut sk) = (vec![0u8; ffi::PUBLIC_KEY_BYTES], vec![0u8; ffi::SECRET_KEY_BYTES]);
    // SAFETY: eK@nonos.systems - buffers sized for the C, mode set under PQ.
    unsafe {
        ffi::NONOS_mldsa65_deterministic_random(1);
        ffi::PQCLEAN_MLDSA65_CLEAN_crypto_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr());
        ffi::NONOS_mldsa65_deterministic_random(0);
    }
    pk
}

#[test]
fn a_failed_draw_refuses_the_key_and_the_signature_but_not_the_reproducible_one() {
    let key = MlDsa65::keygen_random().unwrap();
    let forced = Forced::on();
    let rep = MlDsa65::sign_reproducible(&key.seed, b"m");
    let (kp, sig) = (MlDsa65::keygen_random(), MlDsa65::sign(&key.seed, b"m"));
    drop(forced);
    assert!(kp.is_err() && sig.is_err(), "nothing is made from a failed draw");
    assert!(rep.is_ok(), "the deterministic signature draws nothing");
    assert!(MlDsa65::keygen_random().is_ok(), "a later draw that works is used");
}

#[test]
fn the_zero_mode_never_reaches_a_key_pair() {
    let zero = zero_key();
    let key = MlDsa65::keygen_random().unwrap();
    let signer = std::thread::spawn(move || {
        (0..200).for_each(|_| assert!(MlDsa65::sign_reproducible(&key.seed, b"m").is_ok()))
    });
    let made: Vec<_> = (0..40).map(|_| MlDsa65::keygen_random().unwrap().pubkey).collect();
    signer.join().unwrap();
    assert!(made.iter().all(|pk| *pk != zero));
    assert!(made.windows(2).all(|w| w[0] != w[1]));
}
