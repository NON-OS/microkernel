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

//! The video list is the shared Table at 1x, 1.25x, 1.5x and 2x: the row a
//! click or the hover names is the row the Table draws there, counted in the
//! whole list as it scrolls, and the page holds as many rows as the Table
//! draws whole. Its own binary, so setting the scale races no other test.

use apps_proofs::ui::screen::Route;
use apps_proofs::ui::target::Target;
use apps_proofs::ui::targets::{targets, Shown};
use apps_proofs::ui::view::list_geom::{row_rect, rows_visible};
use apps_proofs::video_check::{hit, points, table, windows};
use nonos_toolkit::design::scale::set_scale;

fn lists() -> impl Iterator<Item = Shown> {
    let page = |s: &Shown| matches!(s.route, Route::Library | Route::Files);
    windows().into_iter().filter(move |s| page(s) && !s.grid)
}

fn every_row_named_is_the_row_the_table_draws_there(q: u32) {
    for s in lists() {
        let rows = targets(&s).into_iter().filter_map(|(t, r)| match t {
            Target::Row(i) => Some((i, r)),
            _ => None,
        });
        let rows: Vec<_> = rows.collect();
        assert_eq!(rows.len(), rows_visible(table(&s)).min(s.len - s.scroll), "q={q}: {s:?}");
        for (i, r) in rows {
            let given = (s.scroll, s.len - s.scroll);
            let all = points(r).iter().all(|&p| hit(table(&s), given, p) == Some(i));
            assert!(all, "q={q}: {i} {s:?}");
        }
    }
}

fn the_page_holds_as_many_rows_as_the_table_draws_whole(q: u32) {
    for s in lists().filter(|s| s.len > 0) {
        let (at, n) = (table(&s), rows_visible(table(&s)));
        let top = |k: usize| (row_rect(at, k).x as i32 + 4, row_rect(at, k).y as i32 + 1);
        assert_eq!(hit(at, (0, n + 5), top(n - 1)), Some(n - 1), "q={q}: the last row in {s:?}");
        assert_eq!(hit(at, (0, n + 5), top(n)), None, "q={q}: no row past the page in {s:?}");
    }
}

#[test]
fn the_list_names_the_rows_the_table_draws_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        every_row_named_is_the_row_the_table_draws_there(q);
        the_page_holds_as_many_rows_as_the_table_draws_whole(q);
    }
    set_scale(4);
}
