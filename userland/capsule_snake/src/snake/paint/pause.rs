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
use crate::snake::ui::pause_geom::{action, modal, title, ACTIONS, LABELS};
use crate::snake::ui::text::as_str;

use super::glow;

const HEADING: &str = "Paused";

// The board keeps painting underneath, so everything here has to blend: the
// scrim, the shadow and the panel all land on pixels that already exist.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let r = roles();
    let (w, h) = (fb.width, fb.height);
    glow::scrim(fb);
    let m = modal(w, h);
    glow::shade(fb, m);
    Card::new().draw(fb, m);
    let t = title(w, h);
    text::left(fb, t.x, t.y as i32, HEADING, r.text, metrics().px_title);
    for index in 0..ACTIONS {
        let look = game.hover.look(game.screen, Target::PauseAction(index));
        let face = Button::new(as_str(LABELS[index])).kind(kind(index)).state(look);
        face.draw(fb, action(w, h, index));
    }
}

fn kind(index: usize) -> Kind {
    match index {
        0 => Kind::Primary,
        3 => Kind::Danger,
        _ => Kind::Secondary,
    }
}
