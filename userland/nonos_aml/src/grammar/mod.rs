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

//! The byte-level productions of chapter 20 that every object is built from.

mod cursor;
mod name;
mod name_seg;
mod pkg_length;

pub use cursor::Cursor;
pub use name::{name_string, NamePath};
pub use name_seg::{is_lead_char, is_name_char, is_name_start};
pub use pkg_length::{pkg_end, pkg_value};
