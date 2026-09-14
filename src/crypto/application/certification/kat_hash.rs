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

//! The two hashes, each against its published digest.

use core::sync::atomic::{AtomicBool, Ordering};

use super::vectors::{BLAKE3_EMPTY, SHA3_256_EMPTY};
use super::{AlgorithmStatus, CRYPTO_STATE};

/// Record a verdict once and hand it back.
pub(super) fn settle(flag: &AtomicBool, passed: bool) -> AlgorithmStatus {
    if passed {
        flag.store(true, Ordering::SeqCst);
        CRYPTO_STATE.checks_passed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Pass
    } else {
        CRYPTO_STATE.checks_failed.fetch_add(1, Ordering::SeqCst);
        AlgorithmStatus::Fail
    }
}

pub fn kat_sha3_256() -> AlgorithmStatus {
    let digest = crate::crypto::sha3::sha3_256(b"");
    settle(&CRYPTO_STATE.sha3_256, digest == SHA3_256_EMPTY)
}

/// The previous form hashed one input twice and asked whether the two agreed
/// and were non-zero, which every wrong implementation also manages.
pub fn kat_blake3() -> AlgorithmStatus {
    let digest = crate::crypto::blake3::blake3_hash(b"");
    settle(&CRYPTO_STATE.blake3, digest == BLAKE3_EMPTY)
}
