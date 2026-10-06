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

//! Trailer v4: the v3 path and a STARK proof over the same statement, side by
//! side. The gate checks both and admits only when both pass; this module only
//! reads and writes the container.

mod layout;
mod parse;
mod words;

#[cfg(any(feature = "alloc", test))]
mod encode;

pub use layout::{MAGIC_V4, MAX_PROOF_V4};
pub use parse::{parse_v4, TrailerV4};
pub use words::root_words;

#[cfg(any(feature = "alloc", test))]
pub use encode::encode_v4;
