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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::draw;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::inset;
use nonos_toolkit::widgets::{text, Rect};

use crate::snake::state::Game;
use crate::snake::ui::icon_table;
use crate::snake::ui::metrics::{GAP_TIGHT, ICON_MD, PAD_TIGHT, PX_LABEL, ROW_H, TABLE_HEAD_H};
use crate::snake::ui::rank_geom::{award_row, awards, AWARD_ROWS};

const HEADING: &str = "Awards";

// Ordinal against `rank_geom::AWARD_ROWS`; the id is the index, which is what
// `awards.dat` stores, so an unlocked mark survives a reload.
const LABELS: [&str; AWARD_ROWS] = [
    "First hundred",
    "Thousand club",
    "Full lattice",
    "No walls needed",
    "Ninety seconds",
    "Deep stack",
];

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let muted = roles().muted;
    let (w, h) = (fb.width, fb.height);
    let band = Rect { h: TABLE_HEAD_H, ..inset(awards(w, h), PAD_TIGHT) };
    text::left(fb, band.x, text::centre_top(band.y, band.h, PX_LABEL), HEADING, muted, PX_LABEL);
    for index in 0..AWARD_ROWS {
        entry(game, fb, award_row(w, h, index), index);
    }
}

fn entry(game: &Game, fb: &mut PaintBuffer, r: Rect, index: usize) {
    let theme = roles();
    let unlocked = game.awards.iter().any(|id| *id as usize == index);
    let ink = if unlocked { theme.text.argb() } else { theme.muted.argb() };
    let mark_y = r.y + ROW_H.saturating_sub(ICON_MD) / 2;
    draw(fb, icon_table::award(unlocked), r.x, mark_y, ICON_MD, ink);
    let x = r.x + ICON_MD + GAP_TIGHT;
    let cut = text::truncate(LABELS[index], r.w.saturating_sub(ICON_MD + GAP_TIGHT), PX_LABEL);
    let body = if unlocked { theme.text_2 } else { theme.muted };
    text::left(fb, x, text::centre_top(r.y, r.h, PX_LABEL), &cut, body, PX_LABEL);
}
