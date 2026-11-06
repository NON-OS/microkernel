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
use nonos_toolkit::widgets::{Button, Kind};

use crate::snake::input::target::Target;
use crate::snake::state::{Game, Phase};
use crate::snake::ui::play_geom_rows::{foot, FOOT_BTNS, FOOT_LABELS};
use crate::snake::ui::text::as_str;

const RESUME: &[u8] = b"Resume";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    for index in 0..FOOT_BTNS {
        let look = game.hover.look(game.screen, Target::Foot(index));
        let face = Button::new(as_str(label(game, index))).kind(kind(index)).state(look);
        face.draw(fb, foot(w, h, index));
    }
}

fn label(game: &Game, index: usize) -> &'static [u8] {
    if index == 0 && game.phase == Phase::Paused {
        return RESUME;
    }
    FOOT_LABELS[index]
}

fn kind(index: usize) -> Kind {
    match index {
        2 => Kind::Danger,
        _ => Kind::Secondary,
    }
}
