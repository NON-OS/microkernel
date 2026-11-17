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

//! No two of a table's targets overlap: down any column each one is a single
//! band, a row tall, at 1x, 1.25x, 1.5x and 2x in a window grown by the same
//! scale. Its own binary, so setting the scale races no other test.

use apps_proofs::pm::state::{Screen, State};
use apps_proofs::pm::ui::hit::Target;
use apps_proofs::pm::ui::table_geom::row_h;
use apps_proofs::pm_window::{at, table, window};
use nonos_toolkit::design::scale::set_scale;

/// What each point down `x` names, from `top` for `h`, as runs of one target.
fn bands(state: &State, x: u32, top: u32, h: u32) -> Vec<(Option<Target>, u32)> {
    let mut out: Vec<(Option<Target>, u32)> = Vec::new();
    for y in top..top + h {
        match out.last_mut() {
            Some((last, n)) if *last == at(state, x, y) => *n += 1,
            _ => out.push((at(state, x, y), 1)),
        }
    }
    out
}

#[test]
fn down_any_column_each_target_is_one_band_a_row_tall_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        for screen in [Screen::Processes, Screen::Overview] {
            let (state, (t, _, laid)) = (window(screen, 5), table(screen));
            for (x, _) in laid {
                let all = bands(&state, t.x + x, t.y, t.h).into_iter();
                let named: Vec<_> = all.filter(|b| b.0.is_some()).collect();
                for (target, n) in &named {
                    let once = named.iter().filter(|b| b.0 == *target).count() == 1;
                    assert!(*n == row_h() && once, "q={q}");
                }
            }
        }
    }
    set_scale(4);
}
