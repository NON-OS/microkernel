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
//! Snake's controls: every point of a screen names the one control drawn
//! there, or none. The table of drawn controls the other proofs walk is here.

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use crate::snake::grid::{WIN_H as H, WIN_W as W};
use crate::snake::input::target::{at, Target};
use crate::snake::state::Screen;
use crate::snake::ui::setup_geom_rows::{self as rows, CHIPS, CHIP_ROWS};
use crate::snake::ui::{home_geom as home, over_geom, pause_geom, play_geom_rows as play};
use crate::snake::ui::{rank_geom, setup_geom};

pub(crate) const SCREENS: [Screen; 6] =
    [Screen::Home, Screen::Setup, Screen::Play, Screen::Pause, Screen::Over, Screen::Rank];

/// Every control `s` draws, with the rect its painter draws it in, at the
/// window's opening size, which the hit test reads until a frame lands.
pub(crate) fn drawn(s: Screen) -> Vec<(Target, Rect)> {
    let mut out = Vec::new();
    let mut put = |k, t: fn(usize) -> Target, r: &dyn Fn(usize) -> Rect| {
        out.extend((0..k).map(|i| (t(i), r(i))));
    };
    let chip = |k| rows::chip(W, H, k / CHIPS, k % CHIPS);
    match s {
        Screen::Home => {
            put(home::ACTIONS, Target::HomeAction, &|i| home::action(W, H, i));
            put(home::CARDS, Target::HomeCard, &|i| home::card(W, H, i));
        }
        Screen::Setup => {
            put(CHIP_ROWS * CHIPS, |k| Target::Chip(k / CHIPS, k % CHIPS), &chip);
            put(rows::TOGGLES, Target::Toggle, &|i| rows::toggle_row(W, H, i));
            put(1, |_| Target::Start, &|_| setup_geom::start(W, H));
        }
        Screen::Play => put(play::FOOT_BTNS, Target::Foot, &|i| play::foot(W, H, i)),
        Screen::Pause => put(4, Target::PauseAction, &|i| pause_geom::action(W, H, i)),
        Screen::Over => put(3, Target::OverAction, &|i| over_geom::action(W, H, i)),
        Screen::Rank => put(1, |_| Target::RankBack, &|_| rank_geom::back(W, H)),
    }
    out
}

#[test]
fn every_point_names_the_one_control_drawn_there_or_none() {
    for s in SCREENS {
        let controls = drawn(s);
        for &(t, r) in &controls {
            assert_eq!(at(s, (r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32), Some(t));
        }
        let grid =
            (0..W as i32).step_by(3).flat_map(|x| (0..H as i32).step_by(3).map(move |y| (x, y)));
        for (x, y) in grid {
            let under: Vec<_> = controls.iter().filter(|(_, r)| contains(*r, x, y)).collect();
            assert!(under.len() <= 1, "two controls are drawn at {x}, {y}");
            assert_eq!(at(s, x, y), under.first().map(|(t, _)| *t), "at {x}, {y}");
        }
    }
}
