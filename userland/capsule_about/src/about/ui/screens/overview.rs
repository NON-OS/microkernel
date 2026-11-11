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
use nonos_toolkit::widgets::Rect;

use crate::about::state::State;

use super::super::gauge;
use super::super::metrics::metrics;
use super::{overview_cards, overview_hero, overview_tiles};

fn tiles_y() -> u32 {
    let m = metrics();
    m.hero_h + m.card_gap
}

fn cards_y() -> u32 {
    tiles_y() + gauge::height() + metrics().card_gap
}

// The stack is fixed, so its extent is a sum rather than a measurement: hero,
// gauge row, then the two half-width cards. Nothing here wraps, which is why this
// screen reports a height the pane can hold and never scrolls.
pub fn content_h(_rect: &Rect) -> u32 {
    cards_y() + overview_cards::height()
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    overview_hero::paint(&mut pane, y, rect.w, state.admission);
    overview_tiles::paint(&mut pane, y + tiles_y() as i32, rect.w, state.held);
    overview_cards::paint(&mut pane, y + cards_y() as i32, rect.w);
}
