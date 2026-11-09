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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Controls, Rect as Area, SectionHeader};

use super::look::item;
use crate::browser::ui::rect::{area, Rect};
use crate::browser::ui::sidebar_geom::{self, Row, RowKind};

/// What the sidebar shows besides its geometry: the row standing for what
/// the window shows, how many tabs are open, each group's name and tab
/// count, and whether the window already holds all the groups it may.
pub struct Look<'a> {
    pub on: Option<RowKind>,
    pub tabs: usize,
    pub groups: &'a [(&'a str, usize)],
    pub full: bool,
}

/// Lays every row out in `c`, New group disabled once the window is full,
/// then draws the Tab Groups heading and each row: icon only in the rail,
/// the Tabs row and each group with its count, a group's name after the
/// slot its colour dot fills. Returns the rows it laid out.
pub fn paint(
    fb: &mut PaintBuffer,
    rail: bool,
    side: Rect,
    k: &Look,
    c: &mut Controls<RowKind>,
) -> Vec<Row> {
    let (rows, n) = (sidebar_geom::rows(rail, side, k.groups.len()), k.groups.len());
    c.clear();
    for r in &rows {
        match r.kind == RowKind::NewGroup && k.full {
            true => c.push_disabled(area(r.rect), r.kind),
            false => c.push(area(r.rect), r.kind),
        };
    }
    if let Some(h) = sidebar_geom::groups_head(rail, side, n) {
        let (at, pad) = (area(h), scale().px(u32::from(SPACE.md)));
        let at = Area { x: at.x + pad, w: at.w.saturating_sub(pad), ..at };
        SectionHeader::new("Tab Groups").caption().draw(fb, at);
    }
    for r in &rows {
        item(fb, r, rail, k, c);
    }
    rows
}
