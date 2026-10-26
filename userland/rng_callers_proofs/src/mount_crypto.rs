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

// The kernel's src/crypto, included into the crate root: the callers' own
// random API over the stand-in RNG, and the real ciphers, hashes and KDFs
// they use. Each module not written out here is the kernel's file.
#[allow(clippy::all)]
#[path = "../../../src/crypto"]
pub mod crypto {
    pub mod error;
    pub use error::{CryptoError, CryptoResult};
    pub use hash::{blake3, sha512};
    pub use symmetric::chacha20poly1305;
    pub use util::{constant_time, constant_time::secure_zero, hmac, rng};

    pub mod symmetric {
        pub mod chacha20poly1305;
    }

    pub mod hash {
        pub mod blake3;
        #[path = "unified/sha256.rs"]
        mod sha256_of;
        pub mod sha512;
        pub use blake3::blake3_hash;
        pub use sha256_of::sha256;
        pub type Hash256 = [u8; 32];
    }

    pub mod util {
        pub use crate::stand_in::rng;
        pub mod argon2;
        pub mod constant_time;
        pub mod hmac;
    }

    pub mod random_api {
        mod basic;
        mod entropy_check;
        mod hardware_mix;
        pub use basic::{get_bytes_secure, get_vec_secure};
    }

    #[path = "core/api/random/api.rs"]
    mod core_random;
    pub use core_random::{fill_random, secure_random_u32, secure_random_u64, secure_random_u8};

    #[path = "core/api/entropy.rs"]
    mod entropy_estimate;
    pub use entropy_estimate::estimate_entropy;

    pub use crate::stand_in::keygen::{generate_secure_key, hash_memory_region};

    pub mod application {
        pub mod certification {
            pub mod kat;
            pub use crate::stand_in::selftest::certification::{AlgorithmStatus, CRYPTO_STATE};
            pub use crate::stand_in::selftest::{kat_aead, kat_hash, kat_sign};
            pub use kat::kat_rng;
        }
    }
}
