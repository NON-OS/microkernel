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

//! The market capsule's verifier as it runs in an image: the ed25519 check
//! and its verdict. The offline-verify stand-in is left out; no image here
//! builds it.

#[path = "../../../userland/capsule_market/src/verify/crypto.rs"]
mod crypto;
#[path = "../../../userland/capsule_market/src/verify/trait_def.rs"]
mod trait_def;

pub use crypto::CryptoVerifier;
pub use trait_def::{Verdict, Verifier};
