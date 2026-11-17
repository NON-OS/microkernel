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

//! Where the titlebar strip puts each control: the tabs from the left, the
//! new-tab button after the last of them, and the tool buttons at the right.
//! The painter and the click both read their boxes from here.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{SPACE, TABS_H};
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Rect, Tabs};

/// The tool buttons, left to right: a new tab, the command palette, the
/// next theme. Every one of them does something.
pub const TOOLS: [IconId; 3] = [IconId::Plus, IconId::Search, IconId::Theme];
/// The width, in logical pixels, the strip asks for per tab.
const TAB_W: u32 = 160;

/// The strip's width before the frame clamps it, for `tabs` tabs.
pub fn nominal_w(tabs: usize) -> u32 {
    let s = scale();
    let tabs = s.px(TAB_W).saturating_mul(tabs as u32);
    tabs.saturating_add(s.px(TABS_H).saturating_mul(TOOLS.len() as u32 + 1))
}

/// The tool buttons, each a square the strip's height, flush right.
pub fn tools(w: u32, h: u32) -> [Rect; 3] {
    let base = w.saturating_sub(h.saturating_mul(TOOLS.len() as u32));
    core::array::from_fn(|i| Rect { x: base + i as u32 * h, y: 0, w: h, h })
}

/// The box the tabs are laid in, leaving room for the new-tab button.
pub fn tab_area(w: u32, h: u32) -> Rect {
    let gap = scale().px(u32::from(SPACE.xs));
    Rect { x: 0, y: 0, w: w.saturating_sub(h.saturating_mul(TOOLS.len() as u32 + 1) + gap), h }
}

/// The new-tab button, a square as tall as a tab, just after the last one.
pub fn plus(tabs: &Tabs, w: u32, h: u32, n: usize) -> Rect {
    let area = tab_area(w, h);
    let gap = scale().px(u32::from(SPACE.xs));
    let last = n.checked_sub(1).and_then(|i| tabs.tab(area, i));
    let (x, y, side) = last.map_or((0, 0, h), |b| (b.x + b.w + gap, b.y, b.h));
    Rect { x: x.min(area.x + area.w + gap), y, w: side, h: side }
}

/// The close mark at the end of a tab: the padding after its label. A tab
/// squeezed narrower than its two paddings has no room for one.
pub fn close(body: Rect) -> Option<Rect> {
    let w = scale().px(u32::from(SPACE.md));
    let x = body.x.checked_add(body.w.checked_sub(w.saturating_mul(2))?)?.saturating_add(w);
    Some(Rect { x, w, ..body })
}
