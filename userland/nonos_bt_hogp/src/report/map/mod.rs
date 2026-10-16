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

mod bits;
mod decode;
mod error;
mod field;
mod global;
mod item;
mod keys;
mod layout;
mod leds;
mod local;
mod main_item;
mod parse;
mod parser;
mod span;

pub use decode::{held, HELD_MAX};
pub use error::MapError;
pub use field::Field;
pub use keys::{has_keys, types, CONSUMER, KEYBOARD, LEDS};
pub use layout::{Layout, IDS_MAX, REPORT_BITS_MAX};
pub use leds::leds;
pub use local::SPANS_MAX;
pub use parse::{parse, MAP_MAX};
pub use parser::{DEPTH_MAX, FIELDS_MAX, STACK_MAX};
pub use span::{Dir, Span};
