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

//! HID usage decoding. Printable keys map to the US base character for
//! their physical position and resolve through the shared layout tables
//! (nonos_keymap), so USB and PS/2 keyboards agree on every layout, shift
//! and caps rule. Control keys keep their ASCII control codes.

mod ascii;
mod caps_lock;
mod resolve;
mod us_base;

pub use ascii::ascii;
pub use caps_lock::is_caps_lock;
pub use resolve::resolve_code;
