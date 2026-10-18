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

//! The CTA-861 video identification codes, from the data file that holds
//! the standard's table, checked and laid out when the crate compiles.

use super::vic_parse::parse;

/// One VIC: its picture, nominal rate and pixel clock. The rate is the
/// integer one (60, not 59.94); the clock is the one at that rate. The
/// width of a pixel-repeated format is the one on the wire (1440 for
/// 480i), as the table lists it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vic {
    pub width: u16,
    pub height: u16,
    pub hz: u16,
    pub interlaced: bool,
    pub pixel_khz: u32,
}

impl Vic {
    pub const NONE: Vic = Vic { width: 0, height: 0, hz: 0, interlaced: false, pixel_khz: 0 };
}

static TABLE: [Vic; 256] = parse(include_bytes!("../../data/cta861_vics.txt"));

/// The format a VIC names, None for a code the table does not list.
pub fn vic(code: u8) -> Option<Vic> {
    let v = TABLE[code as usize];
    (v.width != 0).then_some(v)
}
