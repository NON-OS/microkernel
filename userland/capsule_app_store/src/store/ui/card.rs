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

//! One listing, drawn as a card.

use nonos_app_skeleton::PaintBuffer;

use crate::store::listing::{Listing, Source};
use crate::store::theme::{
    BUTTON_BG, BUTTON_FG, BUTTON_OFF_BG, BUTTON_OFF_FG, CARD_BG, CARD_SEL_BG, CARD_SEL_EDGE, MUTED,
    TILE_COMMUNITY, TILE_LINUX, TILE_NONOS, TITLE,
};

use super::geometry::action_rect;
use super::metrics::{CARD_H, CARD_PAD, NAME_PX, SMALL_PX, TILE, TILE_GAP};
use super::text;

pub fn paint(fb: &mut PaintBuffer, l: &Listing, x: u32, y: u32, w: u32, selected: bool) {
    fb.fill_rect(x, y, w, CARD_H, if selected { CARD_SEL_BG } else { CARD_BG });
    if selected {
        /*
         * A rule down the leading edge rather than a full border: it marks the
         * row without boxing every card on the screen.
         */
        fb.fill_rect(x, y, 3, CARD_H, CARD_SEL_EDGE);
    }

    let tile_y = y + (CARD_H - TILE) / 2;
    fb.fill_rect(x + CARD_PAD, tile_y, TILE, TILE, tint(l.source));
    let initial = [l.name.first().copied().unwrap_or(b'?').to_ascii_uppercase()];
    let ix = x + CARD_PAD + (TILE - text::width_of(&initial, NAME_PX)) / 2;
    text::line(fb, ix, text::top_of(tile_y as i32, TILE, NAME_PX), &initial, TITLE, NAME_PX);

    let text_x = x + CARD_PAD + TILE + TILE_GAP;
    text::line(fb, text_x, y as i32 + 12, &l.name, TITLE, NAME_PX);
    text::line(fb, text_x, y as i32 + 34, l.source.label(), MUTED, SMALL_PX);

    action(fb, l, action_rect(x, y, w));
}

fn action(fb: &mut PaintBuffer, l: &Listing, (x, y, w, h): (u32, u32, u32, u32)) {
    let (bg, fg, word): (u32, u32, &[u8]) = match l.ready {
        true => (BUTTON_BG, BUTTON_FG, b"Install"),
        false => (BUTTON_OFF_BG, BUTTON_OFF_FG, b"Details"),
    };
    fb.fill_rect(x, y, w, h, bg);
    let tx = x + (w - text::width_of(word, SMALL_PX)) / 2;
    text::line(fb, tx, text::top_of(y as i32, h, SMALL_PX), word, fg, SMALL_PX);
}

fn tint(source: Source) -> u32 {
    match source {
        Source::NonOs => TILE_NONOS,
        Source::Linux => TILE_LINUX,
        Source::Community => TILE_COMMUNITY,
    }
}
