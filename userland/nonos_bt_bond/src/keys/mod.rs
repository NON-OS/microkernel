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

//! Where bond records are kept: the keyring's record operations, as a
//! trait, and a store in this capsule's own memory for the boot.

mod memory;
mod types;

pub use memory::{MemoryKeys, MEMORY_RECORDS};
pub use types::{Keys, KeysError, KIND_BOND, KIND_LOCAL_IRK};
