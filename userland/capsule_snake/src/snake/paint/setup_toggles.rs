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
use nonos_toolkit::widgets::{Row, State, Switch};

use crate::snake::input::target::Target;
use crate::snake::state::{Game, Options};
use crate::snake::ui::setup_geom_rows::{toggle_row, TOGGLES, TOGGLE_LABELS};
use crate::snake::ui::text::as_str;

// Wrap is not a free choice in every mode: Zen forces it on and Classic refuses
// it, so the switch is drawn locked rather than pretending to be live.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let state = [game.opts.obstacles, game.opts.wraps(game.mode), game.opts.powerups];
    let locked = [false, Options::wrap_locked(game.mode), false];
    for index in 0..TOGGLES {
        let at = toggle_row(w, h, index);
        let off = State { disabled: locked[index], ..State::default() };
        let switch = Switch::new(state[index]).state(off);
        let look = game.hover.look(game.screen, Target::Toggle(index));
        let row = Row::new(as_str(TOGGLE_LABELS[index])).trailing(switch.size().0);
        let row = row.state(State { disabled: locked[index], ..look });
        row.draw(fb, at);
        switch.draw(fb, row.trailing_rect(at));
    }
}
