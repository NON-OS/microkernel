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

use super::super::super::node::Ns;
use super::state::Builder;

/// The formatting elements, which the list of active formatting elements
/// reopens after misnested markup closed them.
pub const FORMATTING: &[&str] = &[
    "a", "b", "big", "code", "em", "font", "i", "nobr", "s", "small", "strike", "strong", "tt", "u",
];

impl Builder {
    /// Whether `id` is in the special category (13.2.4.2).
    pub(in super::super) fn is_special(&self, id: usize) -> bool {
        let n = &self.dom.nodes[id];
        let tag = n.tag.as_str();
        match n.ns {
            Ns::Html => special_html(tag),
            Ns::MathMl => matches!(tag, "mi" | "mo" | "mn" | "ms" | "mtext" | "annotation-xml"),
            Ns::Svg => matches!(tag, "foreignObject" | "desc" | "title"),
        }
    }
}

/// The HTML elements in the special category.
fn special_html(tag: &str) -> bool {
    matches!(tag, "address" | "applet" | "area" | "article" | "aside" | "base" | "basefont")
        || matches!(tag, "bgsound" | "blockquote" | "body" | "br" | "button" | "caption" | "center")
        || matches!(tag, "col" | "colgroup" | "dd" | "details" | "dir" | "div" | "dl" | "dt")
        || matches!(tag, "embed" | "fieldset" | "figcaption" | "figure" | "footer" | "form")
        || matches!(tag, "frame" | "frameset" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "head")
        || matches!(tag, "header" | "hgroup" | "hr" | "html" | "iframe" | "img" | "input")
        || matches!(tag, "keygen" | "li" | "link" | "listing" | "main" | "marquee" | "menu")
        || matches!(tag, "meta" | "nav" | "noembed" | "noframes" | "noscript" | "object" | "ol")
        || matches!(tag, "p" | "param" | "plaintext" | "pre" | "script" | "search" | "section")
        || matches!(tag, "select" | "source" | "style" | "summary" | "table" | "tbody" | "td")
        || matches!(tag, "template" | "textarea" | "tfoot" | "th" | "thead" | "title" | "tr")
        || matches!(tag, "track" | "ul" | "wbr" | "xmp")
}
