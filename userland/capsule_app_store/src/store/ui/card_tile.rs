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

//! A card's tile: a square tinted by the listing's source, holding its initial.

use nonos_app_skeleton::PaintBuffer;

use crate::store::listing::{Listing, Source};
use crate::store::theme::{TILE_COMMUNITY, TILE_LINUX, TILE_NONOS, TITLE};

use super::super::metrics::{CARD_H, CARD_PAD, NAME_PX, TILE};
use super::super::text;

pub(super) fn tile(fb: &mut PaintBuffer, l: &Listing, x: u32, y: u32) {
    let tile_y = y + (CARD_H - TILE) / 2;
    fb.fill_rect(x + CARD_PAD, tile_y, TILE, TILE, tint(l.source));
    let initial = [l.name.first().copied().unwrap_or(b'?').to_ascii_uppercase()];
    let ix = x + CARD_PAD + (TILE - text::width_of(&initial, NAME_PX)) / 2;
    text::line(fb, ix, text::top_of(tile_y as i32, TILE, NAME_PX), &initial, TITLE, NAME_PX);
}

fn tint(source: Source) -> u32 {
    match source {
        Source::NonOs => TILE_NONOS,
        Source::Linux | Source::Model => TILE_LINUX,
        Source::Community => TILE_COMMUNITY,
    }
}
