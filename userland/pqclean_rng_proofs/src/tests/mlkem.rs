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

//! The kernel's ML-KEM-768 key pair and encapsulation, with the seeded RNG
//! made to fail: neither hands back anything made from a failed draw.

use crate::crypto::pqc::kyber::{kyber_decaps, kyber_encaps, kyber_keygen, KyberError};
use crate::stand_in::{fail_draws, turn};

#[test]
fn with_a_working_rng_both_sides_share_a_secret() {
    let _t = turn();
    fail_draws(None);
    let kp = kyber_keygen().unwrap_or_else(|_| panic!("key pair"));
    let (ct, ss) = kyber_encaps(&kp.public_key).unwrap_or_else(|_| panic!("encaps"));
    let back = kyber_decaps(&ct, &kp.secret_key).unwrap_or_else(|_| panic!("decaps"));
    assert_eq!(ss, back);
}

#[test]
fn a_failed_draw_refuses_the_key_pair() {
    let _t = turn();
    fail_draws(Some((1, false)));
    assert!(matches!(kyber_keygen(), Err(KyberError::RandomnessFailed)));
    assert!(kyber_keygen().is_ok(), "the next draw works, and is used");
}

#[test]
fn a_failed_draw_refuses_the_encapsulation() {
    let _t = turn();
    fail_draws(Some((2, false)));
    let kp = kyber_keygen().unwrap_or_else(|_| panic!("draw 1 works"));
    assert!(matches!(kyber_encaps(&kp.public_key), Err(KyberError::RandomnessFailed)));
    assert!(kyber_encaps(&kp.public_key).is_ok());
}
