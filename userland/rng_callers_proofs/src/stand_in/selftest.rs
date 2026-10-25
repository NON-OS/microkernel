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

//! What the crypto self-test file names beside its RNG check: the other
//! checks it re-exports, which answer `Unavailable` here, and the self-test
//! state with the fields that file uses.

pub mod certification {
    use core::sync::atomic::{AtomicBool, AtomicU32};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AlgorithmStatus {
        Pass,
        Fail,
        Unavailable,
    }

    pub struct CryptoState {
        pub rng: AtomicBool,
        pub checks_passed: AtomicU32,
        pub checks_failed: AtomicU32,
    }

    pub static CRYPTO_STATE: CryptoState = CryptoState {
        rng: AtomicBool::new(false),
        checks_passed: AtomicU32::new(0),
        checks_failed: AtomicU32::new(0),
    };
}

pub mod kat_aead {
    pub fn kat_chacha20poly1305() -> super::certification::AlgorithmStatus {
        super::certification::AlgorithmStatus::Unavailable
    }
}

pub mod kat_hash {
    pub use super::kat_aead::{
        kat_chacha20poly1305 as kat_blake3, kat_chacha20poly1305 as kat_sha3_256,
    };
}

pub mod kat_sign {
    pub use super::kat_aead::kat_chacha20poly1305 as kat_ed25519;
}
