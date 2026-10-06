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

use crate::browser::css::calc::split_top::{items, words};
use crate::browser::css::color::parse_color;
use crate::browser::css::computed::{Computed, Shadow, ShadowLayer, MAX_SHADOWS};
use crate::browser::css::parse_px::parse_len_f;

/* Paint work grows with the blur radius, so larger radii are held here. */
const MAX_BLUR_PX: f32 = 300.0;
const MAX_OFFSET_PX: f32 = 4000.0;

/* box-shadow: comma layers of `[inset] x y [blur [spread]] [color]` in any
 * token order. The whole value replaces the previous one, inset or not;
 * an invalid layer drops the declaration. Fully transparent layers paint
 * nothing and are not kept; past MAX_SHADOWS the lowest layers go. */
pub(super) fn apply_shadow(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    if name != "box-shadow" {
        return false;
    }
    let v = value.trim();
    if ["none", "initial", "unset"].iter().any(|k| v.eq_ignore_ascii_case(k)) {
        c.shadow = None;
        return true;
    }
    let empty = ShadowLayer { dx: 0, dy: 0, blur: 0, spread: 0, color: 0, inset: false };
    let mut s = Shadow { layers: [empty; MAX_SHADOWS], n: 0 };
    for item in items(v) {
        let Some(layer) = layer(item, fs, c.color) else { return true };
        if layer.color >> 24 != 0 && (s.n as usize) < MAX_SHADOWS {
            s.layers[s.n as usize] = layer;
            s.n += 1;
        }
    }
    c.shadow = (s.n > 0).then_some(s);
    true
}

fn layer(item: &str, fs: u32, current: u32) -> Option<ShadowLayer> {
    let (mut len, mut n, mut color, mut inset) = ([0f32; 4], 0, None, false);
    for w in words(item) {
        if w.eq_ignore_ascii_case("inset") && !inset {
            inset = true;
        } else if let Some(px) = parse_len_f(w, fs).filter(|_| n < 4) {
            len[n] = px;
            n += 1;
        } else if color.is_none() {
            let cur = w.eq_ignore_ascii_case("currentcolor");
            color = Some(if cur { current } else { parse_color(w)? });
        } else {
            return None;
        }
    }
    if n < 2 || len[2] < 0.0 {
        return None;
    }
    let off = |v: f32| v.clamp(-MAX_OFFSET_PX, MAX_OFFSET_PX) as i16;
    let blur = len[2].min(MAX_BLUR_PX) as u16;
    let color = color.unwrap_or(current);
    Some(ShadowLayer { dx: off(len[0]), dy: off(len[1]), blur, spread: off(len[3]), color, inset })
}
