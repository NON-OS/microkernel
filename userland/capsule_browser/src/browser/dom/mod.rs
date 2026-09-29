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

mod attach;
mod builder;
mod clone;
mod create;
mod detach;
mod insert_before;
pub mod limits;
mod measure;
pub mod node;
mod parse;
mod place;
mod push;
pub mod quirks;
mod remove_attr;
mod serialize;
mod set_attr;
mod tree;
mod void;

pub use parse::{parse, parse_fragment};
pub use tree::Dom;
