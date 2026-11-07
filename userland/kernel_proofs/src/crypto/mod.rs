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

// The kernel's constant-time helpers and its own BLAKE3, which the token MAC
// runs on. The kernel's BLAKE3 is checked against the reference crate in
// crypto_proofs.
#[path = "../../../../src/crypto/util/constant_time/mod.rs"]
pub mod constant_time;

pub mod util {
    pub use super::constant_time;
}

#[path = "../../../../src/crypto/hash/blake3/mod.rs"]
pub mod kernel_blake3;

pub use kernel_blake3::{blake3_keyed_hash, Hasher as Blake3Hasher};

// No RNG on the host: the token nonce takes the kernel's counter fallback.
pub mod random_api {
    pub fn get_bytes_secure(_buffer: &mut [u8]) -> Result<(), ()> {
        Err(())
    }
}
