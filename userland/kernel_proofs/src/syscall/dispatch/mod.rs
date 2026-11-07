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

//! The kernel's CryptoRandom handler, mounted with the one dispatch helper
//! its file uses. The kernel's `dispatch::errno` (`util.rs`) and the result
//! type's `errno` build the same value.

pub use super::result::errno;

#[path = "../../../../../src/syscall/dispatch/crypto/random.rs"]
pub mod random;

#[cfg(test)]
mod random_tests;
