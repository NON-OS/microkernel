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

use nonos_toolkit::widgets::Pill;

use super::rect::Rect;
use super::settings_cells::px;
use crate::browser::shell::route::Route;

const BTN: u32 = 40;
const GAP: u32 = 6;
const PAD: u32 = 8;
const PILL_GAP: u32 = 12;
const ROUTE_PAD: u32 = 7;
const TEXT_X: u32 = 38;
const TEXT_GAP: u32 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Back,
    Fwd,
    Reload,
    Home,
    Pill,
    Route,
    Reader,
    Star,
    Tabs,
    Sidebar,
}

/// Every hit target on the toolbar, and the box the address text is drawn
/// in. The painter, the click handler, the caret and the damage map all read
/// this, so a click always lands on what was drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolbarGeom {
    pub back: Rect,
    pub fwd: Rect,
    pub reload: Rect,
    pub home: Rect,
    pub pill: Rect,
    pub route: Rect,
    pub text: Rect,
    pub reader: Rect,
    pub star: Rect,
    pub tabs: Rect,
    pub sidebar: Rect,
}

/// Back, forward, reload and home on the left; reader, the bookmark star,
/// tabs and the sidebar button on the right; and the address pill filling
/// what is left between them with the route badge at its right end, in a
/// slot as wide as the badge naming the longest route.
pub fn buttons(toolbar: Rect) -> ToolbarGeom {
    let (btn, gap, pad, pill_gap) = (px(BTN), px(GAP), px(PAD), px(PILL_GAP));
    let y = toolbar.y + (toolbar.h - btn).max(0) / 2;
    let at = |i: i32| Rect::new(toolbar.x + pad + i * (btn + gap), y, btn, btn);
    let (back, fwd, reload, home) = (at(0), at(1), at(2), at(3));
    let right = |i: i32| Rect::new(toolbar.right() - pad - btn - i * (btn + gap), y, btn, btn);
    let (reader, star, tabs, sidebar) = (right(3), right(2), right(1), right(0));
    let left = home.right() + pill_gap;
    let pill = Rect::new(left, y, (reader.x - pill_gap - left).max(0), btn);
    let ((bw, bh), route_pad) = (badge(), px(ROUTE_PAD));
    let (rw, rh) = (bw.min((pill.w - 2 * route_pad).max(0)), bh.min(btn));
    let route = Rect::new(pill.right() - route_pad - rw, y + (btn - rh) / 2, rw, rh);
    let tx = pill.x + px(TEXT_X);
    let text = Rect::new(tx, y, (route.x - px(TEXT_GAP) - tx).max(0), btn);
    ToolbarGeom { back, fwd, reload, home, pill, route, text, reader, star, tabs, sidebar }
}

fn badge() -> (i32, i32) {
    let size = |r: Route| Pill::new(r.name()).size();
    let w = Route::ALL.iter().map(|&r| size(r).0).max().unwrap_or(0);
    (w as i32, size(Route::Direct).1 as i32)
}
