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

//! Whether a measurement is enrolled under a policy root, checked directly.
//!
//! Everything a gate needs is public: the root is compiled in, the context is
//! computed from what the gate is about to run and grant, and the path is in the
//! trailer. So the gate folds the path itself with the same Poseidon the tree was
//! built with and compares the result with the root. There is no proof system
//! here and no soundness error, only a hash function.

#![cfg_attr(not(test), no_std)]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

mod context;
mod field;
mod leaf;
mod measure;
mod params;
mod poseidon;
mod root;
mod trailer;
mod v4;
mod verify;

#[cfg(any(feature = "alloc", test))]
mod tree;

#[cfg(test)]
mod tests;

#[cfg(kani)]
mod kani_proofs;

pub use context::{boot_context, capsule_context};
pub use field::Fp;
pub use leaf::{context_digest, leaf, leaf_of, pad_leaf, Kind};
pub use measure::measure_digest_hybrid;
pub use params::{Digest, MAX_DEPTH, RATE, WIDTH};
pub use poseidon::Poseidon;
pub use root::root_of;
pub use trailer::{digest_to_bytes, MAGIC};
pub use v4::{parse_v4, root_words, TrailerV4, MAGIC_V4, MAX_PROOF_V4};
pub use verify::verify;

#[cfg(any(feature = "alloc", test))]
pub use tree::Tree;
#[cfg(any(feature = "alloc", test))]
pub use v4::encode_v4;
