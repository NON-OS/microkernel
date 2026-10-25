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

pub use super::kat_aead::kat_chacha20poly1305;
pub use super::kat_hash::{kat_blake3, kat_sha3_256};
pub use super::kat_sign::kat_ed25519;
#[cfg(feature = "pqc-unvetted")]
pub use super::unvetted::{kat_ntru, kat_sphincs};

pub fn kat_rng() -> AlgorithmStatus {
    let mut buf1 = [0u8; 32];
    let mut buf2 = [0u8; 32];

    let drawn = crate::crypto::rng::fill_random_bytes_secure(&mut buf1).is_ok()
        && crate::crypto::rng::fill_random_bytes_secure(&mut buf2).is_ok();

    let buf1_zero = buf1.iter().all(|&b| b == 0);
    let buf2_zero = buf2.iter().all(|&b| b == 0);
    let same = buf1 == buf2;

    if drawn && !buf1_zero && !buf2_zero && !same {
        CRYPTO_STATE.rng.store(true, Ordering::SeqCst);
        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Pass
    } else {
        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Fail
    }
}
