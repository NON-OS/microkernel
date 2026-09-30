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

//! Where `crate::crypto` points for the included kernel source: the
//! kernel's own ChaCha20-Poly1305, constant-time helpers and Argon2, so a sector
//! opened here is opened by the code ring 0 runs, and sealed by it too.

#[path = "../../../src/crypto/util/argon2/mod.rs"]
pub mod argon2;
#[path = "../../../src/crypto/symmetric/chacha20poly1305/mod.rs"]
pub mod chacha20poly1305;
#[path = "../../../src/crypto/util/constant_time/mod.rs"]
pub mod constant_time;

/// The kernel reaches Argon2 as `crypto::util::argon2`.
pub mod util {
    pub use super::argon2;
}

#[cfg(test)]
mod aead_vectors;
#[cfg(test)]
mod argon2_vectors;
#[cfg(test)]
mod chacha_diff;
#[cfg(test)]
mod chacha_vectors;
#[cfg(test)]
mod poly_saturated_diff;
#[cfg(test)]
mod prior_chacha20poly1305;
#[cfg(test)]
mod test_input;
