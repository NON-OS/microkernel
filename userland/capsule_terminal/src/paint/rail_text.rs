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
//! The telemetry's spacing and its two private figures' helpers: the stat
//! line, a muted key with its value at the right end, and the level bar.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::theme::derive::with_alpha;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Align, Ink, Label, Rect, SectionHeader, Separator, Surface};

use super::rail_nav::head_h;

/// The level bar's height, in logical pixels.
pub const BAR_H: u32 = 3;

/// Opacity the filled share is carried at, short of the accent's own.
const BAR_ALPHA: u8 = 0xB4;

pub fn gap() -> u32 {
    scale().px(u32::from(SPACE.sm))
}

/// The inset the telemetry keeps from each side, the padding a `Row` gives
/// its label, so the figures line up under the list names.
pub fn pad() -> u32 {
    scale().px(u32::from(SPACE.lg))
}

pub fn lh() -> u32 {
    Label::new("").size().1
}

/// A section caption under a hairline, returning the first content row.
pub fn head(fb: &mut impl Surface, x: u32, y: u32, w: u32, title: &str) -> u32 {
    Separator::new().soft().draw(fb, Rect { x, y, w, h: scale().hair() });
    SectionHeader::new(title).draw(fb, Rect { x, y, w, h: head_h() });
    y + head_h()
}

/// One telemetry line: the key muted at the left, the figure at the right,
/// the key cut to what the figure leaves. Returns the next line's top.
pub fn stat(fb: &mut impl Surface, x: u32, y: u32, w: u32, k: &str, v: &str) -> u32 {
    let line = Rect { x, y, w, h: lh() };
    let vw = Label::new(v).size().0.min(w);
    let kw = w.saturating_sub(vw.saturating_add(gap()));
    Label::new(k).ink(Ink::Muted).draw(fb, Rect { w: kw, ..line });
    Label::new(v).align(Align::Right).draw(fb, line);
    y + lh()
}

/// A track with the filled share blended over it at less than full
/// opacity, so the bar reads as a level rather than a hard rule.
pub fn bar(fb: &mut impl Surface, x: u32, y: u32, w: u32, pct: u32) {
    let (r, h) = (roles(), scale().px(BAR_H));
    draw::fill(fb, Rect { x, y, w, h }, r.sunken);
    let fill = w * pct.min(100) / 100;
    if fill > 0 {
        fb.blend_rect(x, y, fill, h, with_alpha(r.accent.argb(), BAR_ALPHA));
    }
}
