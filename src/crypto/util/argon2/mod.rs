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

//! Argon2id, RFC 9106, with the BLAKE2b of RFC 7693 it is built on: the
//! key-stretching function for a passphrase-keyed data volume.

mod address;
mod argon2id;
mod blake2b;
mod blake2b_compress;
mod blake2b_consts;
mod block;
mod derive;
mod ends;
mod fill;
mod hprime;
mod index;
mod memory;
mod params;
mod segment;
mod wipe;

pub use argon2id::argon2id;
pub use derive::argon2;
pub use ends::Inputs;
pub use params::{Argon2Error, Params, MAX_M_KIB, MAX_P, MAX_T, RECOMMENDED};
