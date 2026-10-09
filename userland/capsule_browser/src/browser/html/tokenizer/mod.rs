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

//! The HTML tokenizer (WHATWG 13.2.5).

mod appropriate;
mod attr;
mod attr_value;
mod cdata;
mod comment;
mod data;
mod doctype;
mod doctype_end;
mod doctype_id;
mod limits;
mod lower;
mod markup;
mod next;
mod raw;
mod run;
mod script;
mod script_scan;
mod script_states;
mod script_step;
mod script_temp;
mod state;
mod tag;
mod token;

pub use limits::MAX_TAG_ATTRS;
pub use state::{TextMode, Tokenizer};
pub use token::{Doctype, Tag, Token};
