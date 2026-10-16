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

//! Where a line of text sits in a box, and the text it can show.

use nonos_toolkit::font::ttf;

use super::scale::scaled;

/// Height of one line box at `px`.
pub fn line_h(px: f32) -> u32 {
    ttf::line_height(scaled(px)).max(0) as u32
}

/// Top edge that centres one line of `px` text inside a box of `box_h`.
pub fn top_y_centered(box_y: u32, box_h: u32, px: f32) -> u32 {
    box_y + box_h.saturating_sub(line_h(px)) / 2
}

/// The valid UTF-8 prefix of `bytes`, so malformed state renders short instead of blank.
pub fn valid_str(bytes: &[u8]) -> &str {
    core::str::from_utf8(bytes)
        .unwrap_or_else(|e| core::str::from_utf8(&bytes[..e.valid_up_to()]).unwrap_or(""))
}
