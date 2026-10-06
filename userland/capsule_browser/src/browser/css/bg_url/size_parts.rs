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

use crate::browser::css::calc::split_top::{items, words};
use crate::browser::css::calc::{eval_value, V};
use crate::browser::css::computed::{BgLen, BgSize};

/// The first layer of a background-size: cover, contain, or a width and
/// an optional height, each auto, a length or a percentage.
pub(in crate::browser::css) fn bg_size(value: &str, fs: u32) -> Option<BgSize> {
    let ws: Vec<&str> = words(items(value).next()?).take(2).collect();
    match ws.first()?.to_ascii_lowercase().as_str() {
        "cover" => return Some(BgSize::Cover),
        "contain" => return Some(BgSize::Contain),
        _ => {}
    }
    let w = side(ws[0], fs)?;
    let h = ws.get(1).and_then(|v| side(v, fs)).unwrap_or(BgLen::Auto);
    Some(if (w, h) == (BgLen::Auto, BgLen::Auto) { BgSize::Auto } else { BgSize::Wh(w, h) })
}

fn side(w: &str, fs: u32) -> Option<BgLen> {
    if w.eq_ignore_ascii_case("auto") {
        return Some(BgLen::Auto);
    }
    match eval_value(w, fs as f32)? {
        V::Len { px, pml } if pml == 0.0 && px > 0.0 => Some(BgLen::Px(px.min(65535.0) as u16)),
        V::Len { px, pml } if px == 0.0 && pml > 0.0 => Some(BgLen::Pct(pml.min(65535.0) as u16)),
        _ => None,
    }
}
