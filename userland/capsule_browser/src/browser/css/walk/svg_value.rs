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

/* The SVG paint properties the cascade carries, their values made readable
 * by the rasterizer: lengths in px, no quote that closes the attribute. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::parse_px::parse_len_f;

const NAMES: &str = "fill fill-opacity fill-rule stroke stroke-width stroke-opacity \
    stroke-linecap stroke-linejoin stroke-miterlimit stroke-dasharray stroke-dashoffset \
    stop-color stop-opacity opacity display";

/* Winning value per property, in the order each first appeared. */
pub(super) type Won = Vec<(&'static str, String)>;

pub(super) fn paint_name(name: &str) -> Option<&'static str> {
    NAMES.split_ascii_whitespace().find(|n| n.eq_ignore_ascii_case(name.trim()))
}

pub(super) fn put(won: &mut Won, name: &'static str, v: String) {
    match won.iter_mut().find(|(k, _)| *k == name) {
        Some(slot) => slot.1 = v,
        None => won.push((name, v)),
    }
}

/* `v` for property `name`: a length that is not a bare number (1.9vw,
 * clamp(), 1em) becomes px; a dash list does so item by item. */
pub(super) fn value(name: &str, v: &str, em: u32) -> String {
    let v = v.trim().replace('"', "'");
    let len = |t: &str| match t.parse::<f32>() {
        Ok(_) => Some(String::from(t)),
        Err(_) => parse_len_f(t, em).map(|px| format!("{px}")),
    };
    let out = match name {
        "stroke-width" | "stroke-dashoffset" => len(&v),
        "stroke-dasharray" if !v.contains('(') => {
            let items: Option<Vec<String>> =
                v.split([',', ' ']).filter(|t| !t.is_empty()).map(len).collect();
            items.map(|l| l.join(" "))
        }
        _ => None,
    };
    out.unwrap_or(v)
}

/* The `style` text: the computed color first, for currentColor, then the
 * winners; the outer <svg>'s opacity is its CSS box's, painted already. */
pub(super) fn style_text(won: &Won, outer: bool, color: u32) -> alloc::boxed::Box<str> {
    let (a, r, g, b) = (color >> 24, (color >> 16) & 255, (color >> 8) & 255, color & 255);
    let mut out = format!("color:rgba({r},{g},{b},{:.3})", a as f32 / 255.0);
    for (k, v) in won.iter().filter(|(k, _)| !(outer && *k == "opacity")) {
        for part in [";", k, ":", v] {
            out.push_str(part);
        }
    }
    out.into_boxed_str()
}
