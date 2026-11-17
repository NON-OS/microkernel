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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::icons::{draw, IconId};
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{clip, mono_left};
use nonos_toolkit::widgets::{Card, Rect};

use super::metrics::{card_metrics, metrics};
use super::text;

// The toolkit card and its caption line on their own, returning the y where
// the body starts. A card that fills its body with something other than a
// single number calls this instead of `paint`, so every card on the screen is
// the same toolkit card, captioned at the same height whatever it holds.
pub fn head(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, icon: IconId, caption: &[u8]) -> u32 {
    let (r, m, c) = (roles(), metrics(), card_metrics());
    Card::new().draw(fb, Rect { x, y, w, h: c.card_h });
    let left = x + c.card_pad;
    let cap_top = y + c.card_pad;
    let body_h = line_height(m.body_px).max(1) as u32;
    let icon_top = cap_top + body_h.saturating_sub(c.card_icon) / 2;
    draw(fb, icon, left, icon_top, c.card_icon, r.muted.argb());
    text::left(fb, left + c.card_icon + c.card_label_gap, cap_top, caption, r.muted, m.body_px);
    cap_top + body_h + c.card_line_gap
}

// A stat card: icon and caption on the first line, the number in mono with its
// unit on the second, the subcaption alone on the third so a wide value cannot
// squeeze it to a fragment, and whatever height is left
// as a band at the bottom. The band's y is returned rather than assumed, so a
// caller drawing a sparkline into it never re-derives this stack.
pub fn paint(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    icon: IconId,
    caption: &[u8],
    value: &[u8],
    unit: &[u8],
    sub: &[u8],
) -> u32 {
    let (r, value_top) = (roles(), head(fb, x, y, w, icon, caption));
    let (m, c) = (metrics(), card_metrics());
    let (left, px, value_px) = (x + c.card_pad, m.body_px, m.card_value_px);
    let body_h = line_height(px).max(1) as u32;
    let value_h = line_height(value_px).max(1) as u32;
    let after =
        mono_left(fb, left, value_top as i32, text::valid(value), r.text, value_px).max(0) as u32;
    let base = value_top + value_h.saturating_sub(body_h);
    text::left(fb, after + c.card_label_gap, base, unit, r.muted, px);
    let sub_top = value_top + value_h + c.card_line_gap;
    let sub = clip(text::valid(sub), w.saturating_sub(c.card_pad * 2), px).as_bytes();
    text::left(fb, left, sub_top, sub, r.muted, px);
    sub_top + body_h + c.card_line_gap
}
