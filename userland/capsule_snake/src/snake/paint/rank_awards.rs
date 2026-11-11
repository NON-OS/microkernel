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
use crate::snake::ui::metrics::metrics;
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
    let (muted, m) = (roles().muted, metrics());
    let (w, h) = (fb.width, fb.height);
    let band = Rect { h: m.table_head_h, ..inset(awards(w, h), m.pad_tight) };
    let top = text::centre_top(band.y, band.h, m.px_label);
    text::left(fb, band.x, top, HEADING, muted, m.px_label);
    for index in 0..AWARD_ROWS {
        entry(game, fb, award_row(w, h, index), index);
    }
}

fn entry(game: &Game, fb: &mut PaintBuffer, r: Rect, index: usize) {
    let (theme, m) = (roles(), metrics());
    let unlocked = game.awards.iter().any(|id| *id as usize == index);
    let ink = if unlocked { theme.text.argb() } else { theme.muted.argb() };
    let mark_y = r.y + m.row_h.saturating_sub(m.icon_md) / 2;
    draw(fb, icon_table::award(unlocked), r.x, mark_y, m.icon_md, ink);
    let x = r.x + m.icon_md + m.gap_tight;
    let room = r.w.saturating_sub(m.icon_md + m.gap_tight);
    let cut = text::truncate(LABELS[index], room, m.px_label);
    let body = if unlocked { theme.text_2 } else { theme.muted };
    text::left(fb, x, text::centre_top(r.y, r.h, m.px_label), &cut, body, m.px_label);
}
