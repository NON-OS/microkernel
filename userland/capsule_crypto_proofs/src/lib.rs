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

//! Host proofs for the crypto pool's RSA scheme selection.
//!
//! The scheme module below is the file the capsule compiles, not a copy. The
//! vector is a directory authority certificate the network really served, and
//! the signature checked is the one it arrived with.

extern crate alloc;

#[path = "../../capsule_crypto/src/server/handlers/rsa_scheme.rs"]
pub mod rsa_scheme;

mod cert;
mod spki;

#[cfg(test)]
mod tests;

pub use cert::{parse, Certificate, CERT};
pub use spki::wrap_pkcs1;
