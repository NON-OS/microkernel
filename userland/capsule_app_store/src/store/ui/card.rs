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
use crate::store::model_card::model_line;
use crate::store::model_weights::MODEL_WEIGHTS;
use crate::status_wire::Status;
use crate::store::progress::Progress;
use crate::store::route_offer::progress_line;
use crate::store::theme::{
    ACCENT, BUTTON_BG, BUTTON_FG, BUTTON_OFF_BG, BUTTON_OFF_FG, CARD_BG, CARD_SEL_BG,
    CARD_SEL_EDGE, DANGER, MUTED, TILE_COMMUNITY, TILE_LINUX, TILE_NONOS, TITLE,
};

use super::geometry::{action_rect, remove_rect};
use super::metrics::{CARD_H, CARD_PAD, NAME_PX, SMALL_PX, TILE, TILE_GAP};
use super::text;

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

    let tile_y = y + (CARD_H - TILE) / 2;
    fb.fill_rect(x + CARD_PAD, tile_y, TILE, TILE, tint(l.source));
    let initial = [l.name.first().copied().unwrap_or(b'?').to_ascii_uppercase()];
    let ix = x + CARD_PAD + (TILE - text::width_of(&initial, NAME_PX)) / 2;
    text::line(fb, ix, text::top_of(tile_y as i32, TILE, NAME_PX), &initial, TITLE, NAME_PX);

    let text_x = x + CARD_PAD + TILE + TILE_GAP;
    text::line(fb, text_x, y as i32 + 12, &l.name, TITLE, NAME_PX);
    /*
     * A Qwen tier says what it downloads and the memory it needs, and while
     * its model comes, how far it has come (`route_offer::progress_line`).
     */
    match (fetch, model_line(&l.id, MODEL_WEIGHTS)) {
        (Some(s), _) => {
            text::line(fb, text_x, y as i32 + 34, progress_line(&s).as_bytes(), ACCENT, SMALL_PX)
        }
        (None, Some(line)) => text::line(fb, text_x, y as i32 + 34, &line, MUTED, SMALL_PX),
        (None, None) => text::line(fb, text_x, y as i32 + 34, l.source.origin(), MUTED, SMALL_PX),
    };

    action(fb, l, action_rect(x, y, w));
    if l.progress == Progress::Installed {
        let (rx, ry, rw, rh) = remove_rect(x, y, w);
        fb.fill_rect(rx, ry, rw, rh, BUTTON_OFF_BG);
        let word: &[u8] = b"Remove";
        let tx = rx + (rw - text::width_of(word, SMALL_PX)) / 2;
        text::line(fb, tx, text::top_of(ry as i32, rh, SMALL_PX), word, BUTTON_OFF_FG, SMALL_PX);
    }
}

fn action(fb: &mut PaintBuffer, l: &Listing, (x, y, w, h): (u32, u32, u32, u32)) {
    let word = l.progress.button(l.ready);
    let (bg, fg) = match l.progress {
        p if p.uninstalled() && l.ready => (BUTTON_BG, BUTTON_FG),
        Progress::Installed => (BUTTON_BG, BUTTON_FG),
        Progress::Queued | Progress::Installing | Progress::Removing => (BUTTON_OFF_BG, ACCENT),
        p if p.needs_installed_system() => (BUTTON_OFF_BG, MUTED),
        p if p.retryable() => (BUTTON_OFF_BG, DANGER),
        Progress::Refused | Progress::Failed(_) => (BUTTON_OFF_BG, BUTTON_OFF_FG),
        Progress::Idle | Progress::Removed => (BUTTON_OFF_BG, BUTTON_OFF_FG),
    };
    fb.fill_rect(x, y, w, h, bg);
    let tx = x + (w - text::width_of(word, SMALL_PX)) / 2;
    text::line(fb, tx, text::top_of(y as i32, h, SMALL_PX), word, fg, SMALL_PX);
}

fn tint(source: Source) -> u32 {
    match source {
        Source::NonOs => TILE_NONOS,
        Source::Linux | Source::Model => TILE_LINUX,
        Source::Community => TILE_COMMUNITY,
    }
}
