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

//! The table of controls each Snake screen draws, with the rect its painter
//! draws each in, at any window size: what the control proofs walk, at the
//! opening window or at any display scale.

use nonos_toolkit::widgets::Rect;

use crate::snake::input::target::Target;
use crate::snake::state::Screen;
use crate::snake::ui::setup_geom_rows::{self as rows, CHIPS, CHIP_ROWS};
use crate::snake::ui::{home_geom as home, over_geom, pause_geom, play_geom_rows as play};
use crate::snake::ui::{rank_geom, setup_geom};

/// Every screen, in the order the proofs walk them.
pub const SCREENS: [Screen; 6] =
    [Screen::Home, Screen::Setup, Screen::Play, Screen::Pause, Screen::Over, Screen::Rank];

/// Every control `s` draws in a `w` by `h` window, with the rect its painter
/// draws it in.
pub fn drawn(s: Screen, w: u32, h: u32) -> Vec<(Target, Rect)> {
    let mut out = Vec::new();
    let mut put = |k, t: fn(usize) -> Target, r: &dyn Fn(usize) -> Rect| {
        out.extend((0..k).map(|i| (t(i), r(i))));
    };
    let chip = |k| rows::chip(w, h, k / CHIPS, k % CHIPS);
    match s {
        Screen::Home => {
            put(home::ACTIONS, Target::HomeAction, &|i| home::action(w, h, i));
            put(home::CARDS, Target::HomeCard, &|i| home::card(w, h, i));
        }
        Screen::Setup => {
            put(CHIP_ROWS * CHIPS, |k| Target::Chip(k / CHIPS, k % CHIPS), &chip);
            put(rows::TOGGLES, Target::Toggle, &|i| rows::toggle_row(w, h, i));
            put(1, |_| Target::Start, &|_| setup_geom::start(w, h));
        }
        Screen::Play => put(play::FOOT_BTNS, Target::Foot, &|i| play::foot(w, h, i)),
        Screen::Pause => {
            put(pause_geom::ACTIONS, Target::PauseAction, &|i| pause_geom::action(w, h, i))
        }
        Screen::Over => {
            put(over_geom::ACTIONS, Target::OverAction, &|i| over_geom::action(w, h, i))
        }
        Screen::Rank => put(1, |_| Target::RankBack, &|_| rank_geom::back(w, h)),
    }
    out
}
