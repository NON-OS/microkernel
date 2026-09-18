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

use core::sync::atomic::Ordering;

use super::{AlgorithmStatus, CRYPTO_STATE};

pub fn kat_rng() -> AlgorithmStatus {
    let mut buf1 = [0u8; 32];
    let mut buf2 = [0u8; 32];

    crate::crypto::rng::fill_random_bytes(&mut buf1);
    crate::crypto::rng::fill_random_bytes(&mut buf2);

    let buf1_zero = buf1.iter().all(|&b| b == 0);
    let buf2_zero = buf2.iter().all(|&b| b == 0);
    let same = buf1 == buf2;

    if !buf1_zero && !buf2_zero && !same {
        CRYPTO_STATE.rng.store(true, Ordering::SeqCst);
        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Pass
    } else {
        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Fail
    }
}

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
