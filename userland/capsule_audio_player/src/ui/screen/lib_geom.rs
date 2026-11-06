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

//! Library-table geometry. The painter and the target table read these, so a
//! click resolves to the row that was drawn under it at the current scroll.

use nonos_toolkit::widgets::Tabs;

use super::super::geometry::Rect;
use super::super::metrics::{line_h, BODY, PAGE, S3, S4};
use super::super::target::Target;
use super::super::view::LIB_TABS;

const TAB_H: i32 = 36;
/// A track row's height, in the Library and in Search.
pub const ROW_H: i32 = 54;

pub fn tabs_rect(r: Rect) -> Rect {
    Rect::new(r.x, r.y + line_h(PAGE) + line_h(BODY) + S3 + S4, r.w, TAB_H)
}

pub fn head_rect(r: Rect) -> Rect {
    Rect::new(r.x, tabs_rect(r).bottom() + S4, r.w, 28)
}

pub fn list_rect(r: Rect) -> Rect {
    let top = head_rect(r).bottom() + S3;
    Rect::new(r.x, top, r.w, (r.bottom() - top).max(0))
}

pub fn visible(r: Rect) -> usize {
    (list_rect(r).h / ROW_H).max(0) as usize
}

pub fn tab_hit(r: Rect, x: i32, y: i32) -> Option<usize> {
    Tabs::new(&LIB_TABS, 0).hit(tabs_rect(r).tk(), x, y)
}

/// A target per whole row of the list `l`, and one for the strip under the
/// last whole row, which a click has always read as the next row.
pub fn rows(l: Rect, scroll: usize, len: usize) -> impl Iterator<Item = (Target, Rect)> {
    let whole = (l.h / ROW_H).max(0) as usize;
    (0..=whole).filter(move |&slot| scroll + slot < len).map(move |slot| {
        let h = if slot < whole { ROW_H } else { l.h - whole as i32 * ROW_H };
        (Target::Row(scroll + slot), Rect::new(l.x, l.y + slot as i32 * ROW_H, l.w, h))
    })
}
