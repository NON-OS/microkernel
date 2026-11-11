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
//! band, a row tall. And the spans the app draws its marks in are the shared
//! table's own columns, at every width, for the process tables and the matrix.

use nonos_toolkit::widgets::Cell;

use crate::pm::state::{Screen, State};
use crate::pm::ui::hit::Target;
use crate::pm::ui::matrix_geom;
use crate::pm::ui::table_geom::{hit, row_h, specs, COLS_FULL, COLS_OVERVIEW};
use crate::pm_hover_tests::{at, window};
use crate::pm_table_tests::{laid, table};

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
fn down_any_column_each_target_is_one_band_a_row_tall() {
    for screen in [Screen::Processes, Screen::Overview] {
        let (state, (t, _, laid)) = (window(screen, 5), table(screen));
        for (x, _) in laid {
            let named: Vec<_> =
                bands(&state, t.x + x, t.y, t.h).into_iter().filter(|b| b.0.is_some()).collect();
            for (target, n) in &named {
                assert!(*n == row_h() && named.iter().filter(|b| b.0 == *target).count() == 1);
            }
        }
    }
}

#[test]
fn the_spans_the_app_draws_over_are_the_shared_tables_own_columns() {
    for cols in [&COLS_FULL[..], &COLS_OVERVIEW[..]] {
        let specs = specs(cols);
        for w in [200, 431, 520, 724, 1004] {
            let heads = laid(&specs, w);
            for x in 0..w {
                let k = heads.iter().position(|(a, n)| x >= *a && x < a + n);
                assert!(hit(&specs, (w, 400), (0, 0), x as i32, 1) == k.map(Cell::Header));
            }
        }
    }
    for w in [300, 520, 1004, 1400] {
        let heads = laid(&matrix_geom::cols(), w);
        for x in 0..w {
            let k = heads.iter().position(|(a, n)| x >= *a && x < a + n);
            assert!(matrix_geom::hit(w, 400, 0, 0, x as i32, 1) == k.map(Cell::Header));
        }
    }
}
