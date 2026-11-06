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
use nonos_toolkit::widgets::{Button, Card};

use crate::snake::input::target::Target;
use crate::snake::state::Game;
use crate::snake::ui::rank_geom::{awards, back, table};

use super::{rank_awards, rank_kept, rank_rows};

const BACK: &str = "Back";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    Card::new().draw(fb, table(w, h));
    Card::new().draw(fb, awards(w, h));
    rank_rows::paint(game, fb);
    rank_awards::paint(game, fb);
    rank_kept::paint(game, fb);
    let look = game.hover.look(game.screen, Target::RankBack);
    Button::new(BACK).state(look).draw(fb, back(w, h));
}
