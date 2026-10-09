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
use super::counts::bucket;
use super::state::Builder;

/// A boundary of the default scope ("has an element in scope").
pub const DEFAULT_BOUND: u16 = 1 << 8;
/// A boundary of list item scope beyond the default ones: ol, ul.
pub const LIST_BOUND: u16 = 1 << 9;
/// A boundary of button scope beyond the default ones: button.
pub const BUTTON_BOUND: u16 = 1 << 10;
/// A boundary of table scope: html, table, template.
pub const TABLE_BOUND: u16 = 1 << 11;
/// An element in the special category.
pub const SPECIAL: u16 = 1 << 12;
/// An element "reset the insertion mode" stops at.
pub const RESETS: u16 = 1 << 13;

impl Builder {
    /// One word per open element for the scope walks: its name bucket in the
    /// low byte, its scope and category bits above.
    pub(in super::super) fn meta_of(&self, id: usize) -> u16 {
        let n = &self.dom.nodes[id];
        let tag = n.tag.as_str();
        let html = n.ns == Ns::Html;
        let bits = [
            (default_bound(n.ns, tag), DEFAULT_BOUND),
            (html && matches!(tag, "ol" | "ul"), LIST_BOUND),
            (html && tag == "button", BUTTON_BOUND),
            (html && matches!(tag, "html" | "table" | "template"), TABLE_BOUND),
            (self.is_special(id), SPECIAL),
            (html && resets(tag), RESETS),
        ];
        let set = bits.iter().filter(|(on, _)| *on).fold(0, |m, (_, bit)| m | bit);
        set | bucket(tag) as u16
    }

    /// Whether the open element at stack index `i` is special.
    pub(in super::super) fn special_at(&self, i: usize) -> bool {
        self.open_meta[i] & SPECIAL != 0
    }
}

/// The elements every scope stops at, per namespace.
fn default_bound(ns: Ns, tag: &str) -> bool {
    match ns {
        Ns::Html => {
            matches!(tag, "applet" | "caption" | "html" | "table" | "td" | "th" | "marquee")
                || matches!(tag, "object" | "select" | "template")
        }
        Ns::MathMl => matches!(tag, "mi" | "mo" | "mn" | "ms" | "mtext" | "annotation-xml"),
        Ns::Svg => matches!(tag, "foreignObject" | "desc" | "title"),
    }
}

/// The elements that settle "reset the insertion mode appropriately".
fn resets(tag: &str) -> bool {
    matches!(tag, "td" | "th" | "tr" | "tbody" | "thead" | "tfoot" | "caption" | "colgroup")
        || matches!(tag, "table" | "template" | "head" | "body" | "frameset" | "html")
}
