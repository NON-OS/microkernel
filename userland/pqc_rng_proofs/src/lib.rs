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

//! The kernel's post-quantum files, mounted where the kernel has them, over
//! its own SHA-3 and constant-time code. Only the RNG stands in
//! (`stand_in`), so a test can make its draws fail.

#![cfg(test)]

extern crate alloc;

pub mod stand_in;
mod tests;

#[path = "../../../src/crypto"]
pub mod crypto {
    pub use crate::stand_in as rng;

    #[path = "util/constant_time/mod.rs"]
    pub mod constant_time;

    pub mod hash {
        pub mod sha3;
    }
    pub use hash::sha3;

    pub mod pqc {
        pub mod mceliece;
        pub mod ntru;
        pub mod sphincs;
    }
}
