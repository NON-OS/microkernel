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

//! Colours as a program names them, and the palette that turns them into
//! pixels. A cell keeps the name, not the pixel, so a theme change or an
//! OSC 4 recolour reaches text already on screen.

mod name;
mod palette;
mod xterm;

pub use name::{dim, rgb, Color};
pub use palette::Palette;
pub use xterm::xterm_color;
