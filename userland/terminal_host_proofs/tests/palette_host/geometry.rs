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

//! Panel and row placement inside the body.

use crate::layout::Rect;
use crate::palette::geom::{panel, query_row, row, rows_fit, MAX_ROWS};

const BODY: Rect = Rect { x: 240, y: 60, w: 1200, h: 700 };

#[test]
fn the_panel_and_its_rows_stay_inside_the_body() {
    for rows in 0..=MAX_ROWS as u32 {
        let p = panel(BODY, 30, rows);
        assert!(p.x >= BODY.x && p.x + p.w <= BODY.x + BODY.w, "rows {rows} width");
        assert!(p.y >= BODY.y && p.y + p.h <= BODY.y + BODY.h, "rows {rows} height");
        for i in 0..rows_fit(p, 30) {
            let r = row(p, i, 30);
            assert!(r.y + r.h <= p.y + p.h, "rows {rows} row {i} overruns");
            assert!(r.x >= p.x && r.x + r.w <= p.x + p.w);
        }
    }
}

#[test]
fn rows_never_overlap_the_query_row() {
    let p = panel(BODY, 30, 6);
    let q = query_row(p, 30);
    assert!(row(p, 0, 30).y >= q.y + q.h);
    for i in 1..rows_fit(p, 30) {
        let prev = row(p, i - 1, 30);
        assert_eq!(prev.y + prev.h, row(p, i, 30).y, "row {i}");
    }
}

#[test]
fn a_body_too_small_for_the_panel_degrades_instead_of_wrapping() {
    for h in 0..120u32 {
        let body = Rect { x: 0, y: 0, w: 200, h };
        let p = panel(body, 30, 8);
        assert!(p.y + p.h <= body.y + body.h, "h {h}");
        assert!(p.x + p.w <= body.w, "h {h} width");
        assert!(rows_fit(p, 30) * 30 <= p.h);
    }
}
