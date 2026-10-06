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

//! Colours as 0xRRGGBB for each name a cell can carry.

use super::name::{rgb, Color};
use super::xterm::xterm_color;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Palette {
    pub colors: [u32; 256],
    pub fg: u32,
    pub bg: u32,
    pub cursor: u32,
}

impl Palette {
    pub fn xterm() -> Palette {
        let mut colors = [0u32; 256];
        for (i, c) in colors.iter_mut().enumerate() {
            *c = xterm_color(i as u8);
        }
        Palette { colors, fg: 0xE5E5E5, bg: 0x000000, cursor: 0xE5E5E5 }
    }

    /// The pixel for `c` in a foreground slot. `bright` is bold text asking
    /// for the bright variant of the first eight colours, as xterm draws it.
    pub fn fg_of(&self, c: Color, bright: bool) -> u32 {
        match c {
            Color::Default => self.fg,
            Color::Indexed(i) if bright && i < 8 => self.colors[i as usize + 8],
            Color::Indexed(i) => self.colors[i as usize],
            Color::Rgb(r, g, b) => rgb(r, g, b),
        }
    }

    pub fn bg_of(&self, c: Color) -> u32 {
        match c {
            Color::Default => self.bg,
            Color::Indexed(i) => self.colors[i as usize],
            Color::Rgb(r, g, b) => rgb(r, g, b),
        }
    }
}

impl Default for Palette {
    fn default() -> Palette {
        Palette::xterm()
    }
}
