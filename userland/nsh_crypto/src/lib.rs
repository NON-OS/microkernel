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

//! The nsh/1 cryptography. The caller brings every random byte: the
//! client's X-Wing seed and the server's encapsulation randomness, so the
//! crate reads no entropy of its own and builds the same for a capsule and
//! a host. Every secret it holds is wiped when dropped.

#![no_std]

mod challenge;
mod count;
mod error;
mod kem;
mod labels;
mod opener;
#[cfg(feature = "proofs")]
mod proofs_hooks;
mod schedule;
mod sealer;
mod secret;
mod transcript;

pub use challenge::{attest_challenge, key_digest};
pub use count::{MAX_SEALED_BYTES, MAX_SEALED_FRAMES};
pub use error::CryptoError;
pub use kem::{encapsulate, ClientKem, CT_LEN, EK_LEN};
pub use labels::{labelled, Label};
pub use opener::Opener;
pub use schedule::{handshake_keys, session_keys, Keys};
pub use sealer::Sealer;
pub use secret::Secret;
pub use transcript::Transcript;
