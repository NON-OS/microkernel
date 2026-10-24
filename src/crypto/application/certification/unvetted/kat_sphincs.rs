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

//! A round trip of SPHINCS+ , built only with the `pqc-unvetted`
//! feature (see `crypto::pqc::unvetted`). It is not a known-answer test.

use core::sync::atomic::Ordering;
use super::super::{AlgorithmStatus, CRYPTO_STATE};

pub fn kat_sphincs() -> AlgorithmStatus {
    let message = b"NONOS SPHINCS+ KAT";

    match crate::crypto::sphincs::sphincs_keygen() {
        Ok(keypair) => match crate::crypto::sphincs::sphincs_sign(&keypair.secret_key, message) {
            Ok(signature) => {
                let valid = crate::crypto::sphincs::sphincs_verify(
                    &keypair.public_key,
                    message,
                    &signature,
                );

                if valid {
                    CRYPTO_STATE.sphincs.store(true, Ordering::SeqCst);
                    CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
                    AlgorithmStatus::Pass
                } else {
                    CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                    AlgorithmStatus::Fail
                }
            }
            Err(_) => {
                CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
                AlgorithmStatus::Fail
            }
        },
        Err(_) => AlgorithmStatus::Unavailable,
    }
}
