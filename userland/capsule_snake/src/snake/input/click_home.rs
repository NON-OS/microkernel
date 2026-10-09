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

use nonos_app_skeleton::EventOutcome;

use crate::snake::state::{daily, Game, Screen};

use super::nav;

// Play and Settings both open the New Run panel: it is the one surface that
// owns mode, difficulty and the rules, and Start is what commits a run.
pub fn action(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 | 3 => nav::go(game, Screen::Setup),
        1 if nav::resumable(game) => nav::resume(game),
        2 => nav::go(game, Screen::Rank),
        _ => EventOutcome::Idle,
    }
}

// The daily card sets up the day's run, the mode and difficulty it names, on
// the New Run panel, where Start commits it as for any run. The best-run card
// opens the ranks it is the top of.
pub fn card(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => {
            let (mode, level) = daily::pick(game.last_ms);
            game.mode = mode;
            game.diff = level;
            nav::go(game, Screen::Setup)
        }
        1 => nav::go(game, Screen::Rank),
        _ => EventOutcome::Idle,
    }
}
