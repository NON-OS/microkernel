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

//! What Linux refuses of an mmap before it looks for room, from the
//! arguments alone, and where a hint asks the mapping to go. Pure, so the
//! host proofs hold every refusal and its errno.

mod args;
pub mod flags;

pub use args::{hint, map_args};
pub use flags::{exact, MAP_FIXED, MAP_FIXED_NOREPLACE};
