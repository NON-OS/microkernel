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

use crate::browser::css::{Computed, PseudoText};

use super::abs_out_of_flow::out_of_flow;
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

/* A list item leads with its marker: the ordinal in an <ol>, a bullet
 * elsewhere, unless list-style-type: none suppressed it. An li displayed
 * inline, inline-block, flex or grid is no list item and has none. A
 * ::marker rule styles it (colour, font, size) and its content replaces
 * the text. */
#[inline(never)]
pub(super) fn add_marker(
    w: &mut Walk,
    item: &ElementIn,
    style: &Computed,
    kids: &mut Vec<BoxNode>,
) {
    let list_item = style.is_block && !style.is_flex && !style.is_grid;
    if item.c.tag != "li" || style.list_none || !list_item {
        return;
    }
    let styled = w.pseudos[item.ch].iter().find(|p| p.kind == PseudoText::MARKER);
    let marker = match styled.and_then(|p| p.text.clone()) {
        Some(text) => text,
        None if item.parent_tag == "ol" => format!("{}. ", item.ordinal),
        None => String::from("\u{2022} "),
    };
    *w.count += 1;
    let mut node = leaf(BoxKind::Text(marker), style, &None, item.ch);
    if let Some(p) = styled {
        node.style = p.style;
    }
    attach_marker(kids, node);
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
