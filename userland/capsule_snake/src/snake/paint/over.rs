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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Button, Card, Kind};

use crate::snake::input::target::Target;
use crate::snake::state::Game;
use crate::snake::ui::metrics::metrics;
use crate::snake::ui::over_geom::{action, panel, title, ACTIONS, LABELS};
use crate::snake::ui::over_geom_rows::still;
use crate::snake::ui::text::as_str;

use super::{board, board_fit, glow, over_summary};

const HEADING: &str = "Run over";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let r = roles();
    let (w, h) = (fb.width, fb.height);
    glow::scrim(fb);
    let p = panel(w, h);
    glow::shade(fb, p);
    Card::new().draw(fb, p);
    let t = title(w, h);
    text::left(fb, t.x, t.y as i32, HEADING, r.text, metrics().px_title);
    board::paint(game, fb, &board_fit::fit(still(w, h)));
    over_summary::paint(game, fb);
    for index in 0..ACTIONS {
        let kind = if index == 0 { Kind::Primary } else { Kind::Secondary };
        let look = game.hover.look(game.screen, Target::OverAction(index));
        let face = Button::new(as_str(LABELS[index])).kind(kind).state(look);
        face.draw(fb, action(w, h, index));
    }
}
