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

//! Parsing a HID report descriptor into the touchpad's and the mouse's
//! field maps, and decoding an input report through them.

mod decode;
mod field;
mod layout;
mod mouse_layout;
mod multitouch;
mod parse;
mod read_bits;

pub use decode::{decode_mouse, decode_touch, TouchSample};
pub use field::Field;
pub use layout::TouchLayout;
pub use mouse_layout::MouseLayout;
pub use multitouch::{primary, Multitouch, ScreenEvent};
pub use parse::{parse, parse_mouse, Surface};
