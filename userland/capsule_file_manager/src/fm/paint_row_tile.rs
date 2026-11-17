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

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::Surface;

use super::entries::Entry;
use super::file_color::color;
use super::file_kind::kind_of;
use super::icon;
use super::layout::{metrics, px};

// The tile the filetype glyph sits on. It is its own opaque ground, so the
// glyph -- which hollows itself by overpainting with `bg` -- renders against a
// known colour whether or not the row underneath is lit.
const TILE: u32 = 30;
const TILE_X: u32 = 8;
const GLYPH: u32 = 18;

/// Draws the rounded filetype tile at the head of a row and returns the x the
/// text column starts at, so no caller repeats the icon arithmetic.
pub fn row_tile(fb: &mut impl Surface, entry: &Entry, y: u32, left: u32) -> u32 {
    let (tile, tile_x, glyph) = (px(TILE), px(TILE_X), px(GLYPH));
    let (tile_y, ground) = (y + (metrics().row_h - tile) / 2, roles().surface.argb());
    fb.fill_round(left + tile_x, tile_y, tile, tile, px(8), ground);
    let tint = color(kind_of(entry));
    let gx = left + tile_x + (tile - glyph) / 2;
    let gy = tile_y + (tile - glyph) / 2;
    if entry.is_dir {
        icon::folder(fb, gx, gy, glyph, tint, ground);
    } else {
        icon::file(fb, gx, gy, glyph, tint, ground);
    }
    left + tile_x + tile + px(12)
}
