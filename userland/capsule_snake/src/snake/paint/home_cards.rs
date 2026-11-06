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
use nonos_toolkit::icons::IconId;

use crate::snake::input::target::Target;
use crate::snake::state::{daily, Game};
use crate::snake::ui::home_geom::{card as slot, CARD_LABELS};
use crate::snake::ui::icon_table;
use crate::snake::ui::stat::stat;

use super::num;

const NO_RUNS: &[u8] = b"No runs stored yet";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    daily(game, fb, w, h);
    best(game, fb, w, h);
}

// The day's mode and difficulty, from the same pick a click on the card sets
// up (`state::daily`), on the wall clock the game already advances rather
// than a fresh syscall in the paint path.
fn daily(game: &Game, fb: &mut PaintBuffer, w: u32, h: u32) {
    let (pick, level) = daily::pick(game.last_ms);
    let (mark, look) = (icon_table::mode(pick), game.hover.look(game.screen, Target::HomeCard(0)));
    stat(fb, slot(w, h, 0), look, mark, CARD_LABELS[0], pick.name(), level.name());
}

// The highest score among the stored runs, which are kept best first.
fn best(game: &Game, fb: &mut PaintBuffer, w: u32, h: u32) {
    let top = game.runs.iter().max_by_key(|run| run.score);
    let score = num::dec(top.map(|run| run.score).unwrap_or(0));
    let sub = top.map(|run| run.mode.name()).unwrap_or(NO_RUNS);
    let (mark, look) = (IconId::GameTrophy, game.hover.look(game.screen, Target::HomeCard(1)));
    stat(fb, slot(w, h, 1), look, mark, CARD_LABELS[1], score.as_bytes(), sub);
}
