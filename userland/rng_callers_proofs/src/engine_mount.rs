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

//! The kernel's capability engine, the parts the proofs need, with their
//! tests beside them so they reach the engine's private fields. A #[path]
//! file holds its children beside it, so these paths are from src/.

#[path = "../../../src/security/policy/capability/engine/chamber.rs"]
mod chamber;
#[path = "../../../src/security/policy/capability/engine/constructor.rs"]
mod constructor;
#[path = "../../../src/security/policy/capability/engine/memory.rs"]
mod memory;
#[path = "../../../src/security/policy/capability/engine/types.rs"]
pub mod types;
pub use types::CapabilityEngine;
#[path = "engine_tests/mod.rs"]
mod tests;
