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

use crate::pm::state::{Filter, Screen, FILTERS};

use super::super::fit::sidebar_w;
use super::super::metrics::metrics;
use super::super::text;

// The chips narrow the process list, so the row only exists on the screens that
// draw one. Origin, hit test and painter all gate on this one predicate.
fn listed(screen: Screen) -> bool {
    matches!(screen, Screen::Overview | Screen::Processes | Screen::Authority)
}

// The single chip formula. Measured, never a glyph count: the body face is
// proportional, so a label's width is only ever what the rasterizer says.
pub fn chip_w(filter: Filter) -> u32 {
    let m = metrics();
    text::width_of(filter.label(), m.body_px) + m.chip_pad_x * 2
}

pub fn width() -> u32 {
    let bare: u32 = FILTERS.iter().map(|f| chip_w(*f)).sum();
    bare + metrics().chip_gap * (FILTERS.len() as u32 - 1)
}

// The row hangs off the measured screen name rather than a fixed column, so a
// longer title pushes the chips right instead of drawing underneath them.
pub fn origin(fb_w: u32, screen: Screen) -> Option<u32> {
    if !listed(screen) {
        return None;
    }
    let m = metrics();
    let title = text::width_of(screen.nav_label(), m.title_px);
    Some(sidebar_w(fb_w) + m.pane_pad_x + title + m.search_meta_gap)
}

pub fn top() -> u32 {
    let m = metrics();
    m.pane_pad_top + m.head_h.saturating_sub(m.chip_h) / 2
}

pub fn at(fb_w: u32, screen: Screen, x: i32, y: i32) -> Option<Filter> {
    let mut cx = origin(fb_w, screen)?;
    let (y0, m) = (top(), metrics());
    if y < y0 as i32 || y >= (y0 + m.chip_h) as i32 {
        return None;
    }
    for &filter in FILTERS.iter() {
        let w = chip_w(filter);
        if x >= cx as i32 && x < (cx + w) as i32 {
            return Some(filter);
        }
        cx += w + m.chip_gap;
    }
    None
}
