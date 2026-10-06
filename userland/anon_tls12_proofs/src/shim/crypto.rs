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


//! The capsule's crypto module, as much of it as the TLS 1.2 client uses.

#[path = "../../../capsule_net_anon/src/crypto/compare.rs"]
mod compare;
#[path = "../../../capsule_net_anon/src/crypto/digest.rs"]
mod digest;
#[path = "../../../capsule_net_anon/src/crypto/error.rs"]
mod error;
#[path = "../../../capsule_net_anon/src/crypto/prf.rs"]
mod prf;
#[path = "../../../capsule_net_anon/src/crypto/sha384/mod.rs"]
pub mod sha384;
#[path = "../../../capsule_net_anon/src/crypto/wipe.rs"]
pub mod wipe;

pub use compare::equal;
/* The whole surface of the included files, so none of it is dead here. */
pub use digest::{ed25519_verify, sha256};
pub use error::CryptoError;
pub use prf::{hkdf_sha256, hmac_sha256};
