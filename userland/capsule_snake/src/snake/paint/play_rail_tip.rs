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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::widgets::rect::inset;
use nonos_toolkit::widgets::{text, Ink, Label, Progress, Rect, Row, Tone};

use crate::snake::state::Game;
use crate::snake::ui::play_geom_rows::{rail_row, RAIL_HEADS};
use crate::snake::ui::text::as_str;

use super::num;
use super::play_rail::{fact, gauge};

const POWER_HEAD: &[u8] = b"Power";
const LIVES_HEAD: &[u8] = b"Lives";

// Rows past the four heads are the same `rail_row` band the geom already
// defines, so the rail keeps one origin no matter how many facts it carries.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    Row::new(as_str(RAIL_HEADS[3])).draw(fb, rail_row(w, h, 3));
    let (first, second) = (lane(rail_row(w, h, 4)), lane(rail_row(w, h, 5)));
    let blurb = as_str(game.mode.blurb());
    let lines = text::wrap(blurb, first.w, scale().font(TextRole::Body.px()));
    if let Some(&(start, end)) = lines.first() {
        Label::new(&blurb[start..end]).ink(Ink::Muted).draw(fb, first);
    }
    if let Some(&(start, _)) = lines.get(1) {
        Label::new(&blurb[start..]).ink(Ink::Muted).draw(fb, second);
    }
    if game.power_active() {
        let left = num::dec((game.power_left() / 1000) as u32 + 1);
        fact(fb, rail_row(w, h, 6), POWER_HEAD, left.as_bytes());
    }
    lives(game, fb, w, h);
}

/// The row less the pad a shared row keeps each side, so a line of the tip
/// starts where a row's label does.
fn lane(r: Rect) -> Rect {
    Rect { y: r.y, h: r.h, ..inset(r, scale().px(u32::from(SPACE.lg))) }
}

fn lives(game: &Game, fb: &mut PaintBuffer, w: u32, h: u32) {
    let total = game.mode.lives();
    if total < 2 {
        return;
    }
    let bar = Progress::new(game.lives as u32, total as u32).tone(Tone::Ok);
    gauge(fb, rail_row(w, h, 7), LIVES_HEAD, bar);
}
