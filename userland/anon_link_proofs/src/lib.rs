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

//! Host proofs for the link handshake over real relay bytes.

//! The chain check and the cell parsers below are the files the capsule
//! compiles. Ed25519 and SHA-256 come from the kernel's own primitives, so
//! nothing in the path under test is a host substitute.

#[path = "shim/cell.rs"]
pub mod cell;

#[path = "shim/crypto.rs"]
pub mod crypto;

#[path = "shim/link.rs"]
pub mod link;

pub mod vectors;

#[cfg(test)]
mod tests;
