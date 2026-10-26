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

//! The kernel's files, mounted where the kernel has them, so their `crate::`
//! and `super::` paths resolve as they do there. Only the RNG, the entropy
//! source, the time counter and the serial line stand in (`stand_in`).

#![cfg(test)]

mod stand_in;
mod stand_in_hw;
mod tests;

pub use stand_in_hw::{arch, sys};

// The kernel's src/crypto: each module not written out inline is its file.
#[path = "../../../src/crypto"]
pub mod crypto {
    pub mod rng {
        pub use crate::stand_in::fill_random_bytes;
        pub(crate) use crate::stand_in::fill_random_bytes_secure;
    }

    pub mod pqclean_support {
        mod random;
        pub(crate) use random::drew_cleanly;
    }

    pub mod util {
        pub mod rng {
            pub mod entropy {
                pub(crate) use crate::stand_in::get_entropy64_secure;
            }
            pub mod global {
                mod fallback;
                // The kernel keeps these to its own module; the tests reach
                // them from here, its parent, as the kernel's callers do.
                pub fn fill_with_fallback_secure(buf: &mut [u8]) {
                    fallback::fill_with_fallback_secure(buf)
                }
                pub fn weak_u64() -> u64 {
                    fallback::weak_u64()
                }
            }
        }
    }

    pub mod pqc {
        pub mod kyber;

        pub mod ml_dsa_65 {
            pub mod api;
            pub mod constants;
            mod drawn;
            mod ffi;
            pub mod types;
            mod verify_stack;
            pub use api::ml_dsa_65_verify;
            pub use drawn::{ml_dsa_65_keypair, ml_dsa_65_sign};
        }
    }
}
