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

//! The HTML tree builder (WHATWG 13.2.6) over the tokenizer in `html`,
//! filling the same `Dom` arena scripts and layout read.
//!
//! `ops` holds the parser state and the operations the insertion modes
//! share (the stack of open elements, the list of active formatting
//! elements, insertion); the other modules are the modes themselves.

mod body;
mod doc;
mod foreign;
mod modes;
mod ops;
mod table;

pub use doc::fragment::fragment_state;
pub use ops::state::Builder;
