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

//! Hit-testing against the same measured geometry the painter used, so a click
//! always lands on the cell that was drawn under the pointer.

use nonos_toolkit::widgets::Rect;

use super::items::items;
use super::metrics::{panel_at, TitleSpan};
use crate::editor::layout::titlebar_h;
use crate::editor::menu_view::hit;

pub(in crate::editor) enum MenuHit {
    Title(usize),
    Row(usize),
    Outside,
}

pub(in crate::editor) fn menubar_hit(
    spans: &[TitleSpan],
    open: Option<usize>,
    bounds: Rect,
    x: i32,
    y: i32,
) -> MenuHit {
    if let Some(o) = open {
        let rows = items(o);
        if let Some(r) = hit(&rows, panel_at(spans, o, &rows, bounds), x, y) {
            return MenuHit::Row(r);
        }
    }
    if x < 0 || y < 0 {
        return MenuHit::Outside;
    }
    let (x, y) = (x as u32, y as u32);
    if y < titlebar_h() {
        for (i, span) in spans.iter().enumerate() {
            if x >= span.x0 && x < span.x1 {
                return MenuHit::Title(i);
            }
        }
    }
    MenuHit::Outside
}
