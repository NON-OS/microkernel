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

//! The process tables are the shared Table, and a click names what it drew:
//! a header names its column, a row its row under the scroll, and the rule
//! under the header and the room past the last row name nothing.

use nonos_toolkit::widgets::{Col as Column, Rect, Table};

use crate::metrics::{WIN_H, WIN_W};
use crate::pm::state::Screen;
use crate::pm::ui::fit::inspector;
use crate::pm::ui::hit::Target;
use crate::pm::ui::pane_geom::{pane_rect, table_rect};
use crate::pm::ui::table_geom::{row_h, row_y, shown, specs, Col, COLS_FULL, COLS_OVERVIEW};
use crate::pm_hover_tests::{at, window};

/// Where `screen` draws its table, its columns and their spans.
pub(crate) fn table(screen: Screen) -> (Rect, &'static [Col], Vec<(u32, u32)>) {
    let pane = pane_rect(WIN_W, WIN_H, inspector(screen, WIN_W));
    let (t, cols) = match screen {
        Screen::Overview => (table_rect(&pane), &COLS_OVERVIEW[..]),
        _ => (pane, &COLS_FULL[..]),
    };
    (t, cols, laid(&specs(cols), t.w))
}

/// Each column's left edge and width across `w`, as the shared table of
/// `specs` heads them, which is where the app draws over its cells.
pub(crate) fn laid(specs: &[Column], w: u32) -> Vec<(u32, u32)> {
    let (head, at) = (Table::new(specs), Rect { x: 0, y: 0, w, h: w });
    (0..specs.len()).filter_map(|c| head.head_rect(at, c)).map(|h| (h.x, h.w)).collect()
}

#[test]
fn a_header_hit_names_the_column_drawn_under_it_and_the_rule_names_nothing() {
    for screen in [Screen::Processes, Screen::Overview] {
        let (state, (t, cols, laid)) = (window(screen, 0), table(screen));
        assert!(laid[0].0 == 0 && laid.windows(2).all(|p| p[0].0 + p[0].1 == p[1].0));
        assert!(laid.last().map(|(x, w)| x + w) == Some(t.w));
        for dx in 0..t.w {
            let k = laid.iter().position(|(x, w)| dx >= *x && dx < x + w).unwrap();
            for y in [t.y, t.y + row_h() - 1] {
                assert!(at(&state, t.x + dx, y) == Some(Target::Sort(cols[k])));
            }
            assert!(at(&state, t.x + dx, t.y + row_h()).is_none());
        }
    }
}

#[test]
fn a_row_hit_names_its_row_under_the_scroll_and_nothing_past_the_last() {
    let cases = [(Screen::Processes, 0), (Screen::Processes, 33), (Screen::Overview, 37)];
    for (screen, scroll) in cases {
        let (state, (t, _, _)) = (window(screen, scroll), table(screen));
        let n = shown(scroll, 40, t.h);
        for (slot, dy, dx) in (0..n).flat_map(|s| [(s, 0, 0), (s, row_h() - 1, t.w - 1)]) {
            assert!(
                at(&state, t.x + dx, t.y + row_y(slot) + dy) == Some(Target::Row(scroll + slot))
            );
        }
        let below = t.y + row_y(n);
        assert!(below >= t.y + t.h || at(&state, t.x, below).is_none());
    }
}
