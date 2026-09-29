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
use crate::browser::css::color::parse_color;
use crate::browser::css::computed::{BgLayer, Computed};

use super::top_slash::top_slash;

/// The `background` shorthand. Every longhand it names is reset first, as
/// CSS does: the colour may only sit in the final comma layer and is read
/// from a top-level token (never from inside url() or a gradient, and never
/// a position/size pair joined by a top-level slash, while the slash inside
/// rgb(0 0 0 / 50%) is part of the colour); size and repeat come from the
/// layer whose image is painted.
pub(in crate::browser::css) fn apply_background(c: &mut Computed, value: &str, fs: u32) {
    let v = value.trim();
    if v.eq_ignore_ascii_case("inherit") {
        return;
    }
    let last = items(v).last().unwrap_or("");
    c.bg = words(last)
        .filter(|w| top_slash(w).is_none())
        .find_map(|w| {
            if w.eq_ignore_ascii_case("currentcolor") {
                Some(c.color)
            } else {
                parse_color(w)
            }
        })
        .unwrap_or(0);
    let layer = super::image_layer(v).unwrap_or(last);
    c.bg_layer = BgLayer::INITIAL;
    let slash = top_slash(layer);
    if let Some(at) = slash {
        apply_bg_size(c, &layer[at + 1..], fs);
    }
    /* The position is the run of position words, before the size if any. */
    let head = &layer[..slash.unwrap_or(layer.len())];
    let ws: Vec<&str> =
        words(head).filter(|w| super::pos_parts::bg_pos(&[w], fs).is_some()).collect();
    if let Some(p) = super::pos_parts::bg_pos(&ws, fs) {
        c.bg_layer.pos = p;
    }
    c.bg_layer.repeat = !words(layer).any(|w| w.eq_ignore_ascii_case("no-repeat"));
}

/// The first layer of a background-size list; an invalid value changes
/// nothing.
pub(in crate::browser::css) fn apply_bg_size(c: &mut Computed, value: &str, fs: u32) {
    if let Some(s) = super::size_parts::bg_size(value, fs) {
        c.bg_layer.size = s;
    }
}
