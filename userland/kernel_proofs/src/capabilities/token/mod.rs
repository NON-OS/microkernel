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

// The real token MAC: material, sign, verify, the key cell and the
// revocation set, as the syscall gate runs them.

pub(crate) use super::token_types as types;
pub use super::CapabilityToken;

#[path = "../../../../../src/capabilities/token/material.rs"]
pub mod material;
#[path = "../../../../../src/capabilities/token/nonce.rs"]
pub mod nonce;
#[path = "../../../../../src/capabilities/token/revocation.rs"]
pub mod revocation;
#[path = "../../../../../src/capabilities/token/sign.rs"]
pub mod sign;
#[path = "../../../../../src/capabilities/token/signing_key.rs"]
pub mod signing_key;
#[path = "../../../../../src/capabilities/token/validate.rs"]
pub mod validate;
#[path = "../../../../../src/capabilities/token/verify.rs"]
pub mod verify;

pub use revocation::{is_revoked, revoke_token};
pub use sign::sign_token;
pub use signing_key::{set_signing_key, signing_key};
pub use validate::is_token_not_revoked;
pub use verify::verify_token;

/// One key for every proof in this crate; the cell is set once.
pub fn test_key() {
    let _ = set_signing_key(&[0x5a; 32]);
}

/// The kernel's unguessable nonce draw, as the proofs (and the fuzz build,
/// which shares this module) reach it.
pub fn secure_nonce() -> Result<[u8; 16], &'static str> {
    nonce::secure_nonce_128()
}
