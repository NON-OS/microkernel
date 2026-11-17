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

//! Snake's controls at the display scale: at 1x, 1.25x, 1.5x and 2x, in a
//! window grown by the same scale, every control each screen draws sits inside
//! the window, and what a click names at any point is the one control drawn
//! there, or none. The hit test reads the opening window at the scale until a
//! frame lands. Its own binary, so setting the scale races no other test.

use apps_proofs::snake::grid::{WIN_H, WIN_W};
use apps_proofs::snake::input::target::at;
use apps_proofs::snake_drawn::{drawn, SCREENS};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::widgets::rect::contains;

#[test]
fn every_point_names_the_one_control_drawn_there_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h) = (scale().px(WIN_W), scale().px(WIN_H));
        for s in SCREENS {
            let controls = drawn(s, w, h);
            for &(t, r) in &controls {
                assert!(r.x + r.w <= w && r.y + r.h <= h, "q={q}: {t:?} leaves the window");
                assert_eq!(at(s, (r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32), Some(t));
            }
            let grid = (0..w as i32)
                .step_by(3)
                .flat_map(|x| (0..h as i32).step_by(3).map(move |y| (x, y)));
            for (x, y) in grid {
                let under: Vec<_> = controls.iter().filter(|(_, r)| contains(*r, x, y)).collect();
                assert!(under.len() <= 1, "q={q}: two controls are drawn at {x}, {y}");
                assert_eq!(at(s, x, y), under.first().map(|(t, _)| *t), "q={q}: at {x}, {y}");
            }
        }
    }
    set_scale(4);
}
