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

//! Host proofs over the real capsule source, pulled in by path.
//!
//! Every `#[path]` module below is the file the capsule compiles, not a copy.
//! The `shim` modules only give those files the module paths they expect on the
//! host. Expected values come from the network's own reference implementation
//! and from the cipher and hash standards, never from this code.

#[path = "shim/cell.rs"]
pub mod cell;
#[path = "shim/circuit.rs"]
pub mod circuit;
#[path = "../../capsule_net_anon/src/ntor/constants.rs"]
pub mod constants;
#[path = "shim/crypto.rs"]
pub mod crypto;
#[path = "shim/directory.rs"]
pub mod directory;
#[path = "../../capsule_net_anon/src/path/draw.rs"]
pub mod draw;
#[path = "../../capsule_net_anon/src/ntor/inputs.rs"]
pub mod inputs;
#[path = "shim/ntor.rs"]
pub mod ntor;
#[path = "../../capsule_net_anon/src/crypto/sha1/mod.rs"]
pub mod sha1;
#[path = "shim/stream.rs"]
pub mod stream;

/*
 * The encoder was included twice, once here and once inside the directory shim,
 * which compiles two copies of one file into one crate. The shim needs the `mod` so
 * the capsule's `super::base64_encode` resolves there; `batch.rs` reaches it as
 * `crate::base64_encode`, so the crate root re-exports the shim's copy rather than
 * mounting a second one.
 */
pub use directory::base64_encode;

#[path = "../../capsule_net_anon/src/manager/batch.rs"]
pub mod batch;
#[path = "../../capsule_net_anon/src/manager/body_step.rs"]
pub mod body_step;
#[path = "shim/manager.rs"]
pub mod manager;
#[path = "shim/path.rs"]
pub mod path;
/// The capsule's own SHA-256, included so the proofs run the real source rather
/// than a copy that could drift from it.
#[path = "../../capsule_net_anon/src/crypto/sha256/mod.rs"]
pub mod sha256;

mod hex;
#[cfg(test)]
mod ntor_vector;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod vectors;

pub use hex::hex;
