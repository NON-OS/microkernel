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

//! The widths of the status glyphs: battery, network and the magnifier.

use crate::render::ui_font;

const BATT_GLYPH_W_LOGICAL: u32 = 26;
const NET_GLYPH_W_LOGICAL: u32 = 16;
const SEARCH_GLYPH_W_LOGICAL: u32 = 15;

pub(super) fn batt_glyph_w() -> u32 {
    BATT_GLYPH_W_LOGICAL * ui_font::scale()
}

pub(super) fn net_glyph_w() -> u32 {
    NET_GLYPH_W_LOGICAL * ui_font::scale()
}

pub(super) fn search_glyph_w() -> u32 {
    SEARCH_GLYPH_W_LOGICAL * ui_font::scale()
}
