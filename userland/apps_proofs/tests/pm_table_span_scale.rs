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

//! The spans the app draws its marks in are the shared table's own columns,
//! at every width, for the process tables and the matrix, at 1x, 1.25x, 1.5x
//! and 2x. Its own binary, so setting the scale races no other test.

use apps_proofs::pm::ui::matrix_geom;
use apps_proofs::pm::ui::table_geom::{hit, specs, COLS_FULL, COLS_OVERVIEW};
use apps_proofs::pm_window::laid;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::Cell;

#[test]
fn the_spans_the_app_draws_over_are_the_shared_tables_own_columns_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        for specs in [specs(&COLS_FULL), specs(&COLS_OVERVIEW)] {
            for w in [200, 431, 520, 724, 1004, 1600] {
                let heads = laid(&specs, w);
                for x in 0..w {
                    let k = heads.iter().position(|(a, n)| x >= *a && x < a + n);
                    assert!(hit(&specs, (w, 400), (0, 0), x as i32, 1) == k.map(Cell::Header));
                }
            }
        }
        for w in [300, 520, 1004, 1400, 2001] {
            let heads = laid(&matrix_geom::cols(), w);
            for x in 0..w {
                let k = heads.iter().position(|(a, n)| x >= *a && x < a + n);
                assert!(matrix_geom::hit(w, 400, 0, 0, x as i32, 1) == k.map(Cell::Header));
            }
        }
    }
    set_scale(4);
}
