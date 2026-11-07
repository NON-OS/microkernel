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

//! The person's files' objects: numbered, under `/u/` only, the user-files
//! broker's alone (syscall/microkernel/data/user_obj). The kernel makes
//! every path from the number, so a caller names no path and reaches
//! nothing outside `/u/`; no other call's names hold a second `/`, so
//! nothing else reaches into it.

mod get;
mod path;
mod put;

pub use get::{user_read, user_size};
pub use path::OBJECT_MAX;
pub use put::{user_put, user_remove};
