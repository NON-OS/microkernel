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

//! OpenPGP detached signatures (RFC 4880, RFC 9580), verified against a
//! pinned keyring: the provenance a pacman or apt repository gives its files.
//!
//! This crate reads packets and computes what was signed; the arithmetic is
//! the caller's `Verifier`, so a machine keeps one RSA and one Ed25519. Only
//! v4 signatures of binary documents, over SHA-256 or SHA-512, are accepted.
//! Anything else is a `Refusal` with its reason, never a silent pass.

#![no_std]

extern crate alloc;

mod digest;
mod key;
mod keyring;
mod mpi;
mod packet;
mod refusal;
mod sig;
mod subpacket;
mod verify;

pub use key::{Key, Material};
pub use keyring::keys;
pub use refusal::Refusal;
pub use verify::{verify, Verified, Verifier};
