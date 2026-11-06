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

//! The video list is the shared Table: the row a click or the hover names is
//! the row the Table draws there, counted in the whole list as it scrolls,
//! and the page holds as many rows as the Table draws whole.

use nonos_toolkit::widgets::{Cell, Col, Rect, Table};

use crate::ui::frame::region::body;
use crate::ui::screen::Route;
use crate::ui::target::Target;
use crate::ui::targets::{targets, Shown};
use crate::ui::view::list_geom::{list_at, row_rect, rows_visible};
use crate::ui::view::page_geom::content;
use crate::video_check::{points, windows};

/// Where the page draws its Table.
fn table(s: &Shown) -> Rect {
    list_at(content(s.route, body(s.dims.0, s.dims.1)).unwrap())
}

/// The row the Table at `at`, given `n` rows from `first`, has at a point.
fn hit(at: Rect, (first, n): (usize, usize), (x, y): (i32, i32)) -> Option<usize> {
    let (cols, blank) = ([Col::new("Name")], vec![&[][..]; n]);
    match Table::new(&cols).rows(&blank).first(first).hit(at, x, y)? {
        Cell::Row { row, .. } => Some(row),
        Cell::Header(_) => None,
    }
}

fn lists() -> impl Iterator<Item = Shown> {
    let page = |s: &Shown| matches!(s.route, Route::Library | Route::Files);
    windows().into_iter().filter(move |s| page(s) && !s.grid)
}

#[test]
fn every_row_named_is_the_row_the_table_draws_there() {
    for s in lists() {
        let rows = targets(&s).into_iter().filter_map(|(t, r)| match t {
            Target::Row(i) => Some((i, r)),
            _ => None,
        });
        let rows: Vec<_> = rows.collect();
        assert_eq!(rows.len(), rows_visible(table(&s)).min(s.len - s.scroll), "{s:?}");
        for (i, r) in rows {
            let given = (s.scroll, s.len - s.scroll);
            assert!(points(r).iter().all(|&p| hit(table(&s), given, p) == Some(i)), "{i} {s:?}");
        }
    }
}

#[test]
fn the_page_holds_as_many_rows_as_the_table_draws_whole() {
    for s in lists().filter(|s| s.len > 0) {
        let (at, n) = (table(&s), rows_visible(table(&s)));
        let top = |k: usize| (row_rect(at, k).x as i32 + 4, row_rect(at, k).y as i32 + 1);
        assert_eq!(hit(at, (0, n + 5), top(n - 1)), Some(n - 1), "the last row in {s:?}");
        assert_eq!(hit(at, (0, n + 5), top(n)), None, "no row past the page in {s:?}");
    }
}
