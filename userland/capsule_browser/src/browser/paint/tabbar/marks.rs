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
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Rect as Area, Tabs};

use crate::browser::ui::tabs_geom::TabPart;

/// Every box a press on the strip can land on, bottom to top: the tabs
/// `tabs` lays out in `area`, the first of them tab `first`, then each
/// one's close mark, then the plus just after the last tab, cut at the
/// strip's width `strip_w`.
pub fn marks(tabs: &Tabs, area: Area, first: usize, strip_w: u32) -> Vec<(Area, TabPart)> {
    let bodies: Vec<Area> = (0..).map_while(|k| tabs.tab(area, k)).collect();
    let at = |k: usize| first + k;
    let mut out: Vec<(Area, TabPart)> =
        bodies.iter().enumerate().map(|(k, b)| (*b, TabPart::Tab(at(k)))).collect();
    let closes = bodies.iter().enumerate();
    out.extend(closes.filter_map(|(k, b)| close(*b).map(|c| (c, TabPart::Close(at(k))))));
    if let Some(p) = bodies.last().and_then(|b| plus(*b, strip_w)) {
        out.push((p, TabPart::Plus));
    }
    out
}

/// A tab's close mark: a square in the padding after its label. A tab
/// squeezed narrower than its two paddings has none.
fn close(body: Area) -> Option<Area> {
    let side = scale().px(u32::from(SPACE.md));
    let x = body.x.checked_add(body.w.checked_sub(side.saturating_mul(2))?)? + side;
    Some(Area { x, y: body.y + body.h.saturating_sub(side) / 2, w: side, h: side })
}

/// The plus: a square as tall as a tab, `SPACE.xs` after the last one.
fn plus(last: Area, strip_w: u32) -> Option<Area> {
    let x = last.x.saturating_add(last.w).saturating_add(scale().px(u32::from(SPACE.xs)));
    let w = last.h.min(strip_w.saturating_sub(x));
    (w > 0).then_some(Area { x, w, ..last })
}
