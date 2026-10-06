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

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::color::parse_color;
use crate::browser::css::decl::Decl;
use crate::browser::dom::node::Node;

pub(super) type Hints = Vec<Decl>;

pub(super) fn hint(out: &mut Hints, name: &str, value: impl Into<String>) {
    out.push(Decl::new(String::from(name), value.into(), false));
}

/* A legacy colour attribute: a CSS colour, or bare hex digits ("ff6600")
 * as old pages write them. */
pub(super) fn color(v: &str) -> Option<String> {
    let v = v.trim();
    if parse_color(v).is_some() && !v.eq_ignore_ascii_case("transparent") {
        return Some(String::from(v));
    }
    let hex = v.len() == 3 || v.len() == 6;
    (hex && v.bytes().all(|b| b.is_ascii_hexdigit())).then(|| format!("#{v}"))
}

pub(super) fn bgcolor(node: &Node, attr: &str, prop: &str, out: &mut Hints) {
    if let Some(c) = node.attr(attr).and_then(color) {
        hint(out, prop, c);
    }
}

/* A legacy length: digits for px, or a percentage ("85%"); anything the
 * digits are followed by other than '%' is ignored, as browsers do. */
pub(super) fn length(node: &Node, attr: &str, prop: &str, out: &mut Hints) {
    let Some(v) = node.attr(attr).map(str::trim) else { return };
    let digits = v.bytes().take_while(|b| b.is_ascii_digit()).count();
    let Ok(n) = v[..digits].parse::<u32>() else { return };
    let unit = if v[digits..].trim_start().starts_with('%') { "%" } else { "px" };
    hint(out, prop, format!("{}{unit}", n.min(100_000)));
}

/* align on a block: its text alignment. On a div or a table part,
 * center (or middle) is -webkit-center, which centres tables too. */
pub(super) fn align(node: &Node, out: &mut Hints) {
    let Some(a) = node.attr("align").map(|a| a.trim().to_ascii_lowercase()) else { return };
    let webkit =
        matches!(node.tag.as_str(), "div" | "td" | "th" | "tr" | "tbody" | "thead" | "tfoot");
    match a.as_str() {
        "center" | "middle" if webkit => hint(out, "text-align", "-webkit-center"),
        "left" | "right" | "center" | "justify" => hint(out, "text-align", a),
        _ => {}
    }
}
