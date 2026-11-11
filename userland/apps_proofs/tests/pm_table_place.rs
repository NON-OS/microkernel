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

//! The process monitor's marks land where they landed when it re-derived the
//! table's columns itself: every header, cell and text inset the Table now
//! names is the rect the app's own spans and ink gave, for the process tables
//! and the authority matrix, at every width and scale tried. Its own binary,
//! so setting the scale races no other test.

use apps_proofs::pm::ui::matrix_geom::{self, NAME_W};
use apps_proofs::pm::ui::table_cols::{fixed_w, specs, COLS_FULL, COLS_OVERVIEW};
use apps_proofs::pm::ui::table_geom::{row_h, row_y};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::rect::share;
use nonos_toolkit::widgets::{Col, Rect, Table};

/// The app's `table_span::spans` as it stood before the Table placed cells.
fn spans(fixed: &[u32], w: u32) -> Vec<(u32, u32)> {
    let s = scale();
    let want = |f: &u32| (s.px(*f), u32::from(*f == 0));
    let add = |a: (u32, u32), b: (u32, u32)| (a.0.saturating_add(b.0), a.1.saturating_add(b.1));
    let (all, shares) = fixed.iter().map(want).fold((0, 0), add);
    let room = w.saturating_sub(all);
    let (mut seen, mut x, mut out) = ((0, 0), 0u32, Vec::with_capacity(fixed.len()));
    for f in fixed {
        let next = add(seen, want(f));
        let cut = |n: (u32, u32), len: u32, of: u32| share(n.1, len, of) - share(n.0, len, of);
        let a = if w >= all { want(f).0 } else { cut((seen.0, next.0), w, all) };
        let b = cut((seen.1, next.1), room, shares);
        out.push((x, a.saturating_add(b)));
        (x, seen) = (x.saturating_add(a).saturating_add(b), next);
    }
    if let Some(last) = out.last_mut() {
        last.1 = last.1.saturating_add(w.saturating_sub(x));
    }
    out
}

/// The app's `table_span::ink` as it stood before the Table placed cells.
fn ink((x, w): (u32, u32)) -> (u32, u32) {
    let pad = scale().px(u32::from(SPACE.md)).min(w / 2);
    (x.saturating_add(pad), w - pad * 2)
}

/// Every header, cell and inset of `cols` in a table `w` wide, the old way
/// and the Table's way, for the rows from `first` that fit.
fn compare(cols: &[Col], fixed: &[u32], w: u32, first: usize) -> Vec<String> {
    let at = Rect { x: 37, y: 53, w, h: row_y(6) + row_h() / 2 };
    let blank: Vec<&[&str]> = vec![&[]; 9];
    let table = Table::new(cols).rows(&blank).first(first);
    let mut fails = Vec::new();
    for (c, span) in spans(fixed, w).into_iter().enumerate() {
        let head = Rect { x: at.x + span.0, y: at.y, w: span.1, h: row_h() };
        let ink_x = (at.x + ink(span).0, ink(span).1);
        let got = table.head_rect(at, c).map(|h| (h, Table::text_rect(h)));
        if got.map(|(h, t)| (h, (t.x, t.w))) != Some((head, ink_x)) {
            fails.push(format!("w={w} col={c} head: old {head:?} {ink_x:?} new {got:?}"));
        }
        for k in 0..6 {
            let old = (Rect { y: at.y + row_y(k), ..head }, ink_x.0);
            let new = table.cell_rect(at, first + k, c).map(|r| (r, Table::text_rect(r).x));
            if new != Some(old) {
                fails.push(format!("w={w} col={c} row={k}: old {old:?} new {new:?}"));
            }
        }
        if table.cell_rect(at, first + 6, c).is_some() {
            fails.push(format!("w={w} col={c}: a row past the bottom has a rect"));
        }
    }
    fails
}

#[test]
fn the_tables_cells_are_where_the_apps_spans_put_its_marks() {
    let mut fails = Vec::new();
    let mut matrix_fixed = [0; 11];
    matrix_fixed[0] = NAME_W;
    for q in [4, 5, 8] {
        set_scale(q);
        for cols in [&COLS_FULL[..], &COLS_OVERVIEW[..]] {
            let fixed: Vec<u32> = cols.iter().map(|c| fixed_w(*c)).collect();
            for w in [0, 60, 200, 431, 520, 724, 1004, 1333, 1600] {
                fails.extend(compare(&specs(cols), &fixed, w, 3));
            }
        }
        for w in [0, 90, 300, 520, 1004, 1400, 2001] {
            fails.extend(compare(&matrix_geom::cols(), &matrix_fixed, w, 0));
        }
    }
    set_scale(4);
    assert!(fails.is_empty(), "{} differ:\n{}", fails.len(), fails.join("\n"));
}
