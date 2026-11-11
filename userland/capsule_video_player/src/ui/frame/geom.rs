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

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::Rect;

use crate::ui::layout::top;

pub const SIDEBAR_W: u32 = 208;
pub const SIDEBAR_MIN: u32 = 62;
pub const COLLAPSE_AT: u32 = 880;
pub const NAV_H: u32 = 40;
pub const NAV_GAP: u32 = 4;
pub const NAV_TOP: u32 = 92;
pub const PAD: u32 = 12;
pub const STORAGE_H: u32 = 68;

pub fn collapsed(w: u32) -> bool {
    w < scale().px(COLLAPSE_AT)
}

pub fn sidebar_w(w: u32) -> u32 {
    scale().px(if collapsed(w) { SIDEBAR_MIN } else { SIDEBAR_W })
}

pub fn sidebar(w: u32, h: u32) -> Rect {
    Rect { x: 0, y: top(), w: sidebar_w(w), h: h.saturating_sub(top()) }
}

pub fn brand(w: u32) -> Rect {
    let (s, pad) = (scale(), scale().px(PAD));
    Rect { x: pad, y: top() + s.px(18), w: sidebar_w(w).saturating_sub(pad * 2), h: s.px(32) }
}

pub fn nav_item(w: u32, index: usize) -> Rect {
    let (s, pad) = (scale(), scale().px(PAD));
    let inner = sidebar_w(w).saturating_sub(pad * 2);
    Rect {
        x: pad,
        y: top() + s.px(NAV_TOP) + index as u32 * (s.px(NAV_H) + s.px(NAV_GAP)),
        w: inner,
        h: s.px(NAV_H),
    }
}

pub fn storage(w: u32, h: u32) -> Rect {
    let (pad, tall) = (scale().px(PAD), scale().px(STORAGE_H));
    let sw = sidebar_w(w).saturating_sub(pad * 2);
    let y = h.saturating_sub(pad).saturating_sub(tall).max(top());
    Rect { x: pad, y, w: sw, h: tall }
}
