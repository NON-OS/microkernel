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

//! Key repeat for a USB keyboard.
//!
//! A PS/2 keyboard repeats a held key itself, as more make codes. A USB boot
//! keyboard reports only changes, so a held key went down once and nothing
//! more: holding Backspace, an arrow or a letter did one step. The driver
//! repeats the last key pressed, as more presses, after the delay and at the
//! rate a PS/2 keyboard comes up with; a release of that key, or a press of
//! another, ends it.

mod due;
mod repeats;
mod state;
mod timing;

pub use repeats::repeats;
pub use state::KeyRepeat;
pub use timing::{DELAY_MS, LIMIT_MS, RATE_MS};
