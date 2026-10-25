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

// Shared by the unit tests and each pre-seed test, which needs a process of
// its own, as its line is said once. Paths are from src/ or tests/ alike.

#[path = "../src/stand_in.rs"]
mod stand_in;
#[path = "../src/stand_in_rng.rs"]
mod stand_in_rng;
extern crate alloc;
pub use stand_in::{arch, sys};

// The kernel's src/crypto: each module not written out inline is its file.
#[path = "../../../src/crypto"]
pub mod crypto {
    pub mod error;
    pub use util::rng;

    // The core API's random calls; only their callers' paths matter here.
    #[path = "core/api/random/api.rs"]
    pub mod core_random;

    pub mod random_api {
        mod basic;
        mod entropy_check;
        mod hardware_mix;
        pub use basic::{fill_bytes, get_bytes, get_bytes_checked, get_bytes_secure, get_vec_secure};
    }

    pub mod util {
        pub mod misc;
        pub mod rng {
            mod api;
            pub mod error;
            pub use crate::stand_in_rng::*;
            pub use api::{fill_bytes, secure_random_u64};
            pub use error::{RngError, RngResult};
            pub use global::*;
            pub mod global {
                mod fallback;
                mod generate;
                mod range;
                use crate::stand_in_rng::{init, reseed, state};
                pub use generate::*;
                pub use range::{random_range, random_range_secure};
            }
        }
    }
}

// The capability tokens' nonces; only the crypto paths they use matter.
#[path = "../../../src/capabilities/token/nonce.rs"]
pub mod nonce;

#[path = "../../../src/elf"]
pub mod elf {
    pub mod aslr {
        pub mod manager;
        pub use manager::state::AslrManager;
    }
}
