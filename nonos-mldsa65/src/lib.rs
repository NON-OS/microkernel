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

//! ML-DSA-65 (FIPS 204) over the pinned C in third_party/pqclean: key pairs,
//! signatures with an empty context, and verification. The caller supplies
//! the randomness for key pairs and signatures; the crate reads no entropy
//! source of its own, so a capsule and a host use it alike.

#![no_std]

mod api;
mod ffi;
mod pool;
mod rng;
mod secret;
mod sign;

pub use api::{keypair, Error, SecretKey, PUBLIC_LEN, SECRET_LEN, SIG_LEN};
pub use sign::{sign, verify};
