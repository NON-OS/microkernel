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

//! Reading a definition block into the arena. Nothing is executed here: the
//! result is the object tree chapter 20's grammar describes, with each method
//! body kept as bytes until its calls can be told apart from its names.

mod elements;
mod fields;
mod leaf;
mod name_term;
mod object;
mod operand;
mod parser;
mod position;
mod scoped;
mod scopes;
mod string;
mod term;

pub use parser::{body, load};
pub use position::Pos;
pub use scopes::Scopes;
