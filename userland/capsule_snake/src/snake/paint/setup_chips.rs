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
use nonos_toolkit::widgets::{State, Tone};

use crate::snake::input::target::Target;
use crate::snake::state::{difficulty, mode, Game};
use crate::snake::ui::setup_geom_rows::{chip, pill, CHIPS, CHIP_ROWS};

/// Each chip as the shared pill, the one in force selected and in the accent.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    for row in 0..CHIP_ROWS {
        for index in 0..CHIPS {
            let on = selected(game, row, index);
            let look = game.hover.look(game.screen, Target::Chip(row, index));
            let tone = if on { Tone::Accent } else { Tone::Plain };
            let face = pill(row, index).tone(tone).state(State { selected: on, ..look });
            face.draw(fb, chip(w, h, row, index));
        }
    }
}

fn selected(game: &Game, row: usize, index: usize) -> bool {
    if row == 0 {
        mode::ALL[index] == game.mode
    } else {
        difficulty::ALL[index] == game.diff
    }
}
