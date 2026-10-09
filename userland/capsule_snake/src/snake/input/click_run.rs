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

use crate::snake::state::{Game, Screen};

use super::nav;

// Footer: Pause, Restart, Home, in `play_geom_rows::FOOT_LABELS` order. The
// game makes no sound, so the footer offers no sound switch. The last button
// ends the run and goes to the home screen, so it is called Home, as on the
// game-over panel; it read Quit, and left the window open. The window closes
// with its close button.
pub fn foot(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => nav::pause(game),
        1 => restart(game),
        2 => nav::go(game, Screen::Home),
        _ => EventOutcome::Idle,
    }
}

pub fn pause_action(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => nav::resume(game),
        1 => restart(game),
        2 => nav::go(game, Screen::Setup),
        _ => nav::go(game, Screen::Home),
    }
}

pub fn over_action(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => restart(game),
        1 => nav::go(game, Screen::Rank),
        _ => nav::go(game, Screen::Home),
    }
}

pub fn restart(game: &mut Game) -> EventOutcome {
    game.reset(false);
    nav::go(game, Screen::Play)
}
