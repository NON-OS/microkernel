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

//! Ed25519 against the RFC 8032 answer.

use super::kat_hash::settle;
use super::vectors::{ED25519_PUBLIC, ED25519_SEED, ED25519_SIG};
use super::{AlgorithmStatus, CRYPTO_STATE};
use crate::crypto::ed25519::{sign, verify, KeyPair, Signature};

/// The whole RFC 8032 contract for test 1: the seed derives that public key,
/// signing the empty message deterministically produces that signature, and
/// that signature verifies under that public key.
///
/// The previous form generated a fresh keypair, signed, verified, and checked
/// a wrong message failed. That passes on an implementation which agrees with
/// itself and with nothing else, which is the failure that matters: a signer
/// nobody else can verify, or a verifier that accepts what nobody else signed.
pub fn kat_ed25519() -> AlgorithmStatus {
    let keypair = KeyPair::from_seed(ED25519_SEED);
    if keypair.public != ED25519_PUBLIC {
        return settle(&CRYPTO_STATE.ed25519, false);
    }
    if sign(&keypair, b"").to_bytes() != ED25519_SIG {
        return settle(&CRYPTO_STATE.ed25519, false);
    }
    let published = Signature::from_bytes(&ED25519_SIG);
    let accepts_vector = verify(&ED25519_PUBLIC, b"", &published);
    let rejects_other = !verify(&ED25519_PUBLIC, b"wrong message", &published);
    settle(&CRYPTO_STATE.ed25519, accepts_vector && rejects_other)
}
