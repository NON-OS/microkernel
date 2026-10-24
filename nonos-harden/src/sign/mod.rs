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

//! Signing the verdict with the operator's own key, and checking such a
//! signature. The signature is Ed25519 over the exact bytes of the verdict
//! file, so any Ed25519 tool can check it, and the verdict names its signer
//! inside those bytes, so a signature cannot be passed off as another key's.

pub mod key;
pub(crate) mod named;
pub mod refuse;
pub mod verify;

use ed25519_dalek::{Signer, SigningKey};

/// The signature file's contents: the signature in hex, and a newline.
pub fn sign(key: &SigningKey, bytes: &[u8]) -> String {
    hex::encode(key.sign(bytes).to_bytes()) + "\n"
}

/// The signer as the verdict names it: the public key in hex.
pub fn signer(key: &SigningKey) -> String {
    hex::encode(key.verifying_key().to_bytes())
}
