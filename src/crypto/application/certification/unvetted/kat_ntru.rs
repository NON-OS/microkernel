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

//! A round trip of NTRU, built only with the `pqc-unvetted`
//! feature (see `crypto::pqc::unvetted`). It is not a known-answer test.

use core::sync::atomic::Ordering;
use super::super::{AlgorithmStatus, CRYPTO_STATE};

pub fn kat_ntru() -> AlgorithmStatus {
    match crate::crypto::ntru::ntru_keygen() {
        Ok(keypair) => match crate::crypto::ntru::ntru_encaps(&keypair.public_key) {
            Ok((ciphertext, shared_secret1)) => {
                match crate::crypto::ntru::ntru_decaps(&ciphertext, &keypair.secret_key) {
                    Ok(shared_secret2) => {
                        if shared_secret1 == shared_secret2 {
                            CRYPTO_STATE.ntru.store(true, Ordering::SeqCst);
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
