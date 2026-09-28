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

//! Where each primitive the onion layer needs comes from, and why.
//!
//! SHA-256, HMAC, HKDF, X25519, Ed25519 and RSA verify come from the crypto
//! pool through `nonos_libc`, so there is one implementation of each in the
//! system and it is the one the kernel measured. AES-128-CTR comes from
//! `nonos_aes`, shared rather than copied. SHA-1 is the only primitive
//! implemented here, because nothing else in the tree has one and the pool
//! should not grow one: it is in this protocol solely as four legacy integrity
//! bytes per cell.

mod compare;
mod digest;
mod ecdh;
mod error;
mod prf;
mod sha1_once;

pub mod sha1;
pub mod sha256;

pub use compare::equal;
pub use digest::{ed25519_verify, sha256};
pub use ecdh::Ephemeral;
pub use error::CryptoError;
pub use nonos_aes::Ctr128Be;
pub use prf::{hkdf_sha256, hmac_sha256};
pub use sha1_once::sha1_digest;
pub use sha256::digest as sha256_software;
