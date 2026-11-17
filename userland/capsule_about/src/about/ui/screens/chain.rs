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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{centre_top, left, width};

use super::super::metrics::metrics;
use super::super::mono::valid;

/// A chip's side padding and corner radius, the length of a link, and the
/// radius of the dot that ends it, in logical pixels.
const CHIP_PAD_X: u32 = 11;
const CHIP_RADIUS: u32 = 8;
const CHAIN_LINK: u32 = 26;
const LINK_DOT: u32 = 3;

fn pill_w(label: &[u8]) -> u32 {
    scale().px(CHIP_PAD_X) * 2 + width(valid(label), metrics().body_px)
}

// A left-to-right chain of pills joined by hairlines. The last node is the thing
// the chain is about, so it carries the accent and the ones before it do not.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, nodes: &[&[u8]]) {
    let (r, m, s) = (roles(), metrics(), scale());
    let (h, radius, px) = (m.chain_h, s.px(CHIP_RADIUS), m.body_px);
    let mut cx = x;
    let last = nodes.len().saturating_sub(1);
    for (i, node) in nodes.iter().enumerate() {
        let w = pill_w(node);
        let fg = if i == last { r.accent } else { r.text_2 };
        if y >= 0 && y + h as i32 <= fb.height as i32 {
            fb.fill_round(cx, y as u32, w, h, radius, r.raised.argb());
            fb.stroke_round(cx, y as u32, w, h, radius, s.hair(), r.border_strong.argb());
        }
        left(fb, cx + s.px(CHIP_PAD_X), y + centre_top(0, h, px), valid(node), fg, px);
        cx += w;
        if i < last {
            link(fb, cx, y);
            cx += s.px(CHAIN_LINK);
        }
    }
}

fn link(fb: &mut PaintBuffer, x: u32, y: i32) {
    let (r, s) = (roles(), scale());
    let mid = y + (metrics().chain_h / 2) as i32;
    if mid < 0 || mid >= fb.height as i32 {
        return;
    }
    let (link, dot) = (s.px(CHAIN_LINK), s.px(LINK_DOT));
    fb.fill_rect(x, mid as u32, link, s.hair(), r.border.argb());
    fb.circle(x + link - dot, mid as u32, dot, r.border.argb());
}
