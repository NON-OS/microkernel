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

//! What an install will erase and write, or an update keep and write, one
//! row at a time, in the words the installers show before a disk is
//! written. Read off the plan itself, so the screen says what the writer
//! will do.

mod carried;
mod keep_rows;
mod rows;
mod shared;
mod size;

pub use keep_rows::describe_keep;
pub use rows::{describe, Row};
pub use size::size_text;
