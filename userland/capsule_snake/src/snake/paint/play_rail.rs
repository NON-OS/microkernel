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
use nonos_toolkit::widgets::{Card, Progress, Rect, Row};

use crate::snake::state::{level, Game};
use crate::snake::ui::play_geom::rail;
use crate::snake::ui::play_geom_rows::{rail_row, RAIL_HEADS};
use crate::snake::ui::text::as_str;

use super::play_rail_tip;

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    Card::new().draw(fb, rail(w, h));
    fact(fb, rail_row(w, h, 0), RAIL_HEADS[0], game.mode.name());
    fact(fb, rail_row(w, h, 1), RAIL_HEADS[1], level::name(game.level));
    progress(game, fb, rail_row(w, h, 2));
    play_rail_tip::paint(game, fb);
}

/// A shared row naming `head`, with `value` at its right end.
pub fn fact(fb: &mut PaintBuffer, r: Rect, head: &[u8], value: &[u8]) {
    Row::new(as_str(head)).value(as_str(value)).draw(fb, r);
}

/// A shared row naming `head`, with `bar` across the right half of it.
pub fn gauge(fb: &mut PaintBuffer, r: Rect, head: &[u8], bar: Progress) {
    let row = Row::new(as_str(head)).trailing(r.w / 2);
    row.draw(fb, r);
    bar.draw(fb, row.trailing_rect(r));
}

// Progress is the score climbing between this level's threshold and the next,
// and the last level has nowhere left to climb, so the bar reads full.
fn progress(game: &Game, fb: &mut PaintBuffer, r: Rect) {
    let base = level::threshold(game.level);
    let (num, den) = match level::next_threshold(game.level) {
        Some(next) => (game.score.saturating_sub(base), next.saturating_sub(base)),
        None => (1, 1),
    };
    gauge(fb, r, RAIL_HEADS[2], Progress::new(num, den));
}
