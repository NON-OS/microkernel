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

//! What follows each opcode, as data: one row per group of opcodes that share
//! an operand list in chapter 20's grammar, and an index built from the rows
//! at compile time.

mod arg;
mod data;
mod expr;
mod index;
mod object;
mod statement;

pub use arg::{Arg, Class, Group};
pub use index::lookup;
