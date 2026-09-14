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

extern crate alloc;

mod kat;
mod kat_aead;
mod kat_hash;
mod kat_sign;
mod metadata;
mod rfc8439;
mod selftest;
mod vectors;

pub use kat::*;
pub use kat_aead::kat_chacha20poly1305;
pub use kat_hash::{kat_blake3, kat_sha3_256};
pub use kat_sign::kat_ed25519;
pub use metadata::*;
pub use selftest::run_selftest;

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationStatus {
    Certified,
    NotVerified,
    Failed,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmStatus {
    Pass,
    Fail,
    Pending,
    Unavailable,
}

pub struct CryptoState {
    pub sha3_256: AtomicBool,
    pub blake3: AtomicBool,
    pub chacha20poly1305: AtomicBool,
    pub ed25519: AtomicBool,
    pub rng: AtomicBool,
    pub sphincs: AtomicBool,
    pub ntru: AtomicBool,
    pub overall_checks_run: AtomicBool,
    pub checks_passed: AtomicU32,
    pub checks_failed: AtomicU32,
}

impl CryptoState {
    pub const fn new() -> Self {
        Self {
            sha3_256: AtomicBool::new(false),
            blake3: AtomicBool::new(false),
            chacha20poly1305: AtomicBool::new(false),
            ed25519: AtomicBool::new(false),
            rng: AtomicBool::new(false),
            sphincs: AtomicBool::new(false),
            ntru: AtomicBool::new(false),
            overall_checks_run: AtomicBool::new(false),
            checks_passed: AtomicU32::new(0),
            checks_failed: AtomicU32::new(0),
        }
    }
}

pub static CRYPTO_STATE: CryptoState = CryptoState::new();

pub fn get_certification_status() -> CertificationStatus {
    if !CRYPTO_STATE.overall_checks_run.load(Ordering::SeqCst) {
        return CertificationStatus::NotVerified;
    }

    let failed = CRYPTO_STATE.checks_failed.load(Ordering::SeqCst);
    let passed = CRYPTO_STATE.checks_passed.load(Ordering::SeqCst);

    if failed > 0 && passed > 0 {
        CertificationStatus::Degraded
    } else if failed > 0 {
        CertificationStatus::Failed
    } else {
        CertificationStatus::Certified
    }
}
