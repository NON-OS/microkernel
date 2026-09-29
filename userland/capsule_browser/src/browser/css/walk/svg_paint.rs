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

/* SVG paint properties from the page cascade, for inline SVG content. The
 * rasterizer reads an SVG document on its own, so the rules that style an
 * inline <svg> (`.icon path { fill: none; stroke: var(--accent) }`) and a
 * var() inside a presentation attribute reach it only as the resolved
 * `style` text built here: CSS rules over presentation attributes, as SVG
 * 2 orders them, and the element's computed color for currentColor. */

use alloc::borrow::Cow;
use alloc::boxed::Box;

use crate::browser::css::computed::Computed;
use crate::browser::css::vars::{substitute, At};
use crate::browser::dom::node::{Node, Ns};

use super::order::Order;
use super::svg_value::{paint_name, put, style_text, value, Won};

/// The resolved `style` text for SVG element `node`, or None for any other
/// element. `c` is its computed style: its font size and color.
pub(in crate::browser::css) fn svg_paint(
    node: &Node,
    order: &Order,
    mut vars: At,
    c: &Computed,
) -> Option<Box<str>> {
    if node.ns != Ns::Svg {
        return None;
    }
    let em = c.font_size_px;
    let mut won = Won::new();
    for (k, v) in node.attrs.iter().filter(|(_, v)| v.contains("var(")) {
        if let (Some(name), Some(v)) = (paint_name(k), substitute(v, &mut vars)) {
            put(&mut won, name, value(name, &v, em));
        }
    }
    order.each(0, &mut |d| {
        let Some(name) = paint_name(&d.name) else { return };
        let v = match d.needs_resolve() {
            true => substitute(&d.value, &mut vars),
            false => Some(Cow::Borrowed(d.value.as_str())),
        };
        if let Some(v) = v {
            put(&mut won, name, value(name, &v, em));
        }
    });
    Some(style_text(&won, node.tag == "svg", c.color))
}
