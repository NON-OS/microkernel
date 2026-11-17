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

//! The process monitor's window as its hit, hover and table proofs walk it,
//! grown by the display scale in force, and where each screen draws its table.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::{Col as Column, Pointer, Rect, Table};

use crate::metrics::{WIN_H, WIN_W};
use crate::pm::state::{Filter, Row, Screen, State};
use crate::pm::ui::pane_geom::{pane_rect, table_rect};
use crate::pm::ui::table_geom::{specs, Col, COLS_FULL, COLS_OVERVIEW};
use crate::pm::ui::{fit::inspector, hit::Target, hover::target_at};

/// The opening window at the display scale in force, in drawing pixels.
pub fn size() -> (u32, u32) {
    (scale().px(WIN_W), scale().px(WIN_H))
}

/// Forty processes and four findings on `screen` at `scroll`, in `size()`.
pub fn window(screen: Screen, scroll: usize) -> State {
    let (fb_w, fb_h) = size();
    State {
        rows: (0..40).map(|i| Row { pid: i + 100, caps: 0, name: b"app" }).collect(),
        filter: Filter::All,
        flagged: Vec::new(),
        me: 0,
        selected_pid: 0,
        scroll,
        screen,
        alerts: vec![(); 4],
        alert_scroll: 0,
        fb_w,
        fb_h,
        pointer: Pointer::new(),
        hover: None,
    }
}

/// What the click and the hover name at a point of the window.
pub fn at(state: &State, x: u32, y: u32) -> Option<Target> {
    target_at(state, x as i32, y as i32)
}

/// Where `screen` draws its table, its columns and their spans.
pub fn table(screen: Screen) -> (Rect, &'static [Col], Vec<(u32, u32)>) {
    let (w, h) = size();
    let pane = pane_rect(w, h, inspector(screen, w));
    let (t, cols) = match screen {
        Screen::Overview => (table_rect(&pane), &COLS_OVERVIEW[..]),
        _ => (pane, &COLS_FULL[..]),
    };
    (t, cols, laid(&specs(cols), t.w))
}

/// Each column's left edge and width across `w`, as the table heads `specs`.
pub fn laid(specs: &[Column], w: u32) -> Vec<(u32, u32)> {
    let (head, at) = (Table::new(specs), Rect { x: 0, y: 0, w, h: w });
    (0..specs.len()).filter_map(|c| head.head_rect(at, c)).map(|h| (h.x, h.w)).collect()
}
