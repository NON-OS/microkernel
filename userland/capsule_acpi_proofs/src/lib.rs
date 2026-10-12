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

//! Proofs for capsule_acpi's pure parts, built on the host against the
//! capsule's own sources, included by path.

extern crate alloc;

#[path = "../../capsule_acpi/src/server/policy.rs"]
pub mod policy;
#[path = "props.rs"]
pub mod props;

#[cfg(test)]
mod policy_tests;
#[cfg(test)]
mod props_tests;
#[cfg(test)]
mod thread_tests;
