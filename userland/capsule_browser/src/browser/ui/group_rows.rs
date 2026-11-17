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

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{ICON, SPACE};

use super::rect::Rect;
use super::sidebar_geom::{row_h, Row, RowKind};

const HEAD_H: u32 = 28;
const GAP: u32 = 2;

fn head_h() -> i32 {
    scale().px(HEAD_H) as i32
}

fn step() -> i32 {
    row_h() + scale().px(GAP) as i32
}

/// The height of a Tab Groups section that lists `shown` groups: its
/// heading, the All tabs row, the group rows and the New group row.
pub fn height(shown: usize) -> i32 {
    head_h() + (shown as i32 + 2) * step()
}

/// How many of `groups` a section given `room` pixels can list, or None
/// when not even its heading and two fixed rows fit.
pub fn fit(room: i32, groups: usize) -> Option<usize> {
    let spare = room - height(0);
    (spare >= 0).then(|| groups.min((spare / step()) as usize))
}

/// The heading over a section whose top is `top`, across `x..x + w`.
pub fn head(x: i32, w: i32, top: i32) -> Rect {
    Rect::new(x, top, w.max(0), head_h())
}

/// The section's rows under its heading: All tabs, groups `0..shown`, then
/// New group.
pub fn rows(x: i32, w: i32, top: i32, shown: usize) -> Vec<Row> {
    let kinds = core::iter::once(RowKind::AllTabs)
        .chain((0..shown).map(RowKind::Group))
        .chain(core::iter::once(RowKind::NewGroup));
    let (y0, h) = (top + head_h(), row_h());
    let at = |(i, kind): (usize, RowKind)| Row {
        rect: Rect::new(x, y0 + i as i32 * step(), w.max(0), h),
        kind,
    };
    kinds.enumerate().map(at).collect()
}

/// A group row split for its colour dot: a `SPACE.md` dot where a
/// `NavItem` puts its icon, and the box the item is drawn in after it, so
/// the name lines up with the labels of the rows that have icons.
pub fn dot_slot(row: Rect) -> (Rect, Rect) {
    let s = scale();
    let (pad, icon) = (s.px(u32::from(SPACE.md)) as i32, s.px(ICON) as i32);
    let (slot, d) = ((pad + icon).min(row.w), pad.min(row.h));
    let dot = Rect::new(row.x + pad + (icon - d) / 2, row.y + (row.h - d) / 2, d, d);
    (dot, Rect::new(row.x + slot, row.y, row.w - slot, row.h))
}
