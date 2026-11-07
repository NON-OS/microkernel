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

//! Two pure leaf helpers the language core needs. Byte-identical to the
//! capsule's `term::util::{is_space, format_u64}`; kept here so the library
//! has no dependency on the capsule it was carved out of.

mod format_u64;
mod is_space;

pub use format_u64::format_u64;
pub use is_space::is_space;
