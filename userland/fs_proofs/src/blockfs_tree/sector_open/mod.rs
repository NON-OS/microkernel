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

#[path = "../../../../../src/fs/cryptoblock/constants.rs"]
pub mod constants;
#[path = "../../../../../src/fs/cryptoblock/sector_open.rs"]
pub mod sector_open;

mod in_place_store;
mod seal_fixture;
mod tests;
mod tests_in_place;
mod tests_refused;
