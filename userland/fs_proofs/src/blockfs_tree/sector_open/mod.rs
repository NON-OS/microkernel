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

//! Opening a sealed sector with no heap, and a range read that opens whole
//! blocks in place.
//!
//! The sector opener is the shipping kernel file, included by `#[path]`,
//! checked against sectors sealed by the kernel's own `aead_encrypt`.

// The sector constants `crate::fs::cryptoblock` already mounts, so the opener
// and the tree arithmetic share one copy of the kernel file.
pub use crate::fs::cryptoblock as constants;
// The kernel's sector_open.rs, named for what it is here so it does not
// repeat the name of the module that holds it.
#[path = "../../../../../src/fs/cryptoblock/sector_open.rs"]
pub mod opener;

mod in_place_store;
mod seal_fixture;
mod tests;
mod tests_in_place;
mod tests_refused;
