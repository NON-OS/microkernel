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

//! The kernel's attestation files a local trailer passes through.

#[path = "../../../../../src/security/capsule_attest/error.rs"]
pub mod error;

#[path = "../../../../../src/security/capsule_attest/layout.rs"]
pub mod layout;

#[path = "../../../../../src/security/capsule_attest/trailer.rs"]
pub mod trailer;

#[path = "../../../../../src/security/capsule_attest/against_pedersen.rs"]
pub mod against_pedersen;

#[cfg(test)]
mod local_root_refusals;
#[cfg(test)]
mod local_root_tests;
