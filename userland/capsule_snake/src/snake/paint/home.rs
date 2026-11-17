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
use nonos_toolkit::design::size::CONTROL_RADIUS;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Button, Kind, State};

use crate::snake::input::target::Target;
use crate::snake::state::{Game, Phase};
use crate::snake::ui::home_geom::{action, wordmark, ACTIONS, LABELS};
use crate::snake::ui::metrics::metrics;
use crate::snake::ui::text::as_str;

use super::{glow, home_cards};

const TAGLINE: &str = "Steer the line, take the light, stay alive";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    mark(fb, w, h);
    for index in 0..ACTIONS {
        let r = action(w, h, index);
        let (kind, disabled) = style(game, index);
        let look = State { disabled, ..game.hover.look(game.screen, Target::HomeAction(index)) };
        Button::new(as_str(LABELS[index])).kind(kind).state(look).draw(fb, r);
        if index == 0 {
            glow::bloom(fb, r, scale().px(CONTROL_RADIUS));
        }
    }
    home_cards::paint(game, fb);
}

// Continue is only honest while a run is actually suspended behind the hub.
fn style(game: &Game, index: usize) -> (Kind, bool) {
    match index {
        0 => (Kind::Primary, false),
        1 if game.phase == Phase::Ready || game.phase == Phase::GameOver => (Kind::Secondary, true),
        _ => (Kind::Secondary, false),
    }
}

fn mark(fb: &mut PaintBuffer, w: u32, h: u32) {
    let (theme, m) = (roles(), metrics());
    let r = wordmark(w, h);
    text::centre(fb, r, r.y as i32, "SNAKE", theme.text, m.px_wordmark);
    let cut = text::truncate(TAGLINE, r.w, m.px_body);
    let top = r.y + line_height(m.px_wordmark).max(1) as u32;
    text::centre(fb, r, top as i32, &cut, theme.muted, m.px_body);
}
