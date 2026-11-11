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
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{Pill, Rect, Tone};

use crate::snake::state::{Game, Phase};
use crate::snake::ui::metrics::metrics;
use crate::snake::ui::play_geom::{board, stage};

use super::{board as board_paint, play_foot, play_hud, play_rail};

const READY: &str = "Press an arrow key to start";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    play_hud::paint(game, fb);
    board_paint::paint(game, fb, &board(w, h));
    play_rail::paint(game, fb);
    play_foot::paint(game, fb);
    if game.phase == Phase::Ready {
        ready(fb, w, h);
    }
}

// A pill low on the board rather than a full overlay: the snake spawns at the
// board centre, so the hint clears it and the player sees the line to steer.
fn ready(fb: &mut PaintBuffer, w: u32, h: u32) {
    let hint = Pill::new(READY).tone(Tone::Accent);
    let (b, m) = (board(w, h), metrics());
    let r = centre(stage(w, h), hint.size().0, m.btn_h);
    let y = b.y + b.h.saturating_sub(m.btn_h + m.gap_wide);
    hint.draw(fb, Rect { y, ..r });
}
