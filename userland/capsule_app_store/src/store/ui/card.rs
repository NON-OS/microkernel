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

use crate::status_wire::Status;
use crate::store::listing::Listing;
use crate::store::progress::Progress;
use crate::store::theme::{CARD_BG, CARD_SEL_BG, CARD_SEL_EDGE, TITLE};

use super::geometry::{action_rect, remove_rect};
use super::metrics::{CARD_H, CARD_PAD, NAME_PX, TILE, TILE_GAP};
use super::text;

#[path = "card_button.rs"]
mod card_button;
#[path = "card_subtitle.rs"]
mod card_subtitle;
#[path = "card_tile.rs"]
mod card_tile;

use card_button::{action, remove};
use card_subtitle::subtitle;
use card_tile::tile;

pub fn paint(
    fb: &mut PaintBuffer,
    l: &Listing,
    fetch: Option<Status>,
    (x, y, w): (u32, u32, u32),
    selected: bool,
) {
    fb.fill_rect(x, y, w, CARD_H, if selected { CARD_SEL_BG } else { CARD_BG });
    if selected {
        /*
         * A rule down the leading edge rather than a full border: it marks the
         * row without boxing every card on the screen.
         */
        fb.fill_rect(x, y, 3, CARD_H, CARD_SEL_EDGE);
    }

    tile(fb, l, x, y);

    let text_x = x + CARD_PAD + TILE + TILE_GAP;
    text::line(fb, text_x, y as i32 + 12, &l.name, TITLE, NAME_PX);
    subtitle(fb, l, fetch, text_x, y);

    action(fb, l, action_rect(x, y, w));
    if l.progress == Progress::Installed {
        remove(fb, remove_rect(x, y, w));
    }
}
