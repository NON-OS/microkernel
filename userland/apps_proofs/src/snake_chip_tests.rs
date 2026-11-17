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
//! Snake's mode and difficulty chips: each is named exactly where its shared
//! pill paints, which is smaller than the strip it sits in.

use crate::snake::input::target::{at, Target};
use crate::snake::state::Screen;
use crate::snake::ui::setup_geom_rows::pill;
use crate::snake_controls_tests::drawn;

#[test]
fn a_chip_is_named_exactly_where_its_pill_paints() {
    for (t, r) in drawn(Screen::Setup) {
        let Target::Chip(row, i) = t else { continue };
        let (x0, y0) = (r.x as i32 - 2, r.y as i32 - 2);
        let near =
            (x0..x0 + r.w as i32 + 4).flat_map(|x| (y0..y0 + r.h as i32 + 4).map(move |y| (x, y)));
        for (x, y) in near {
            assert_eq!(pill(row, i).hit(r, x, y), at(Screen::Setup, x, y) == Some(t), "{x}, {y}");
        }
    }
}
