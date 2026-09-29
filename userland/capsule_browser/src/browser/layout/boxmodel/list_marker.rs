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

use crate::browser::css::Computed;

use super::abs_out_of_flow::out_of_flow;
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

/* A list item leads with its marker: the ordinal in an <ol>, a bullet
 * elsewhere, unless list-style-type: none suppressed it. */
pub(super) fn add_marker(
    w: &mut Walk,
    item: &ElementIn,
    style: &Computed,
    kids: &mut Vec<BoxNode>,
) {
    if item.c.tag != "li" || style.list_none {
        return;
    }
    let marker = if item.parent_tag == "ol" {
        format!("{}. ", item.ordinal)
    } else {
        String::from("\u{2022} ")
    };
    *w.count += 1;
    attach_marker(kids, leaf(BoxKind::Text(marker), style, &None, item.ch));
}

/* The marker belongs on the list item's first line. As a sibling of a
 * block-level first child (display:block nav links) it would be wrapped
 * into its own anonymous line, so descend through leading in-flow blocks
 * until it can join an inline run. */
fn attach_marker(kids: &mut Vec<BoxNode>, marker: BoxNode) {
    if let Some(first) = kids.first_mut() {
        if matches!(first.kind, BoxKind::Block)
            && !out_of_flow(&first.style)
            && !first.children.is_empty()
        {
            return attach_marker(&mut first.children, marker);
        }
    }
    kids.insert(0, marker);
}
