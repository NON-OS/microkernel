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

use crate::browser::css::{Computed, Size};
use crate::browser::dom::node::Node;

use super::super::edges_x::edges_x;
use super::super::edges_y::edges_y;
use super::inline_word::measure;

/* The input size attribute's default: a field about twenty characters wide. */
const DEFAULT_SIZE: u32 = 20;
/* The widest size honoured; a hostile attribute cannot make a vast field. */
const MAX_SIZE: u32 = 1000;
/* The width of a select's drop-down arrow beside its option text. */
const ARROW_PX: i32 = 20;
/* The side of a checkbox or radio button in the native appearance. */
const TICK_PX: i32 = 13;

/* Give a form control its intrinsic size where CSS leaves it auto: a text
 * field is `size` characters (ch, the digit zero's advance) wide, a select
 * as wide as its widest option plus the arrow, a checkbox or radio a small
 * square, and all of them one line tall. Buttons shrink to their label as
 * any inline-block does, so they are left alone. `widest` is the widest
 * option text of a select. */
pub(in super::super) fn size_field(node: &Node, style: &mut Computed, widest: Option<&str>) {
    let ty = node.attr("type").unwrap_or("text").to_ascii_lowercase();
    if matches!(ty.as_str(), "submit" | "button" | "reset" | "image") {
        return;
    }
    let tick = matches!(ty.as_str(), "checkbox" | "radio") && node.tag == "input";
    let size = node.attr("size").and_then(|v| v.trim().parse::<u32>().ok()).filter(|&n| n > 0);
    let ch = crate::browser::fonts::ch_px(style.font_px);
    let content_w = match (tick, widest) {
        (true, _) => TICK_PX,
        (false, Some(t)) => measure(style, t).saturating_add(ARROW_PX),
        (false, None) => (size.unwrap_or(DEFAULT_SIZE).min(MAX_SIZE) as f32 * ch + 0.5) as i32,
    };
    let content_h = if tick { TICK_PX } else { style.line_height() as i32 };
    let ((el, er), (et, eb)) = (edges_x(style), edges_y(style));
    let bb = |v: i32, e: i32| (if style.border_box { v + e } else { v }).max(0) as u32;
    if style.width == Size::Auto {
        style.width = Size::Px(bb(content_w, el + er));
    }
    if style.height == Size::Auto {
        style.height = Size::Px(bb(content_h, et + eb));
    }
}
