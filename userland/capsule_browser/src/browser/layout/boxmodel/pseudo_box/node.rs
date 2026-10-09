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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::PseudoText;

use super::super::box_kind::box_kind;
use super::super::leaf::leaf;
use super::super::tree::{BoxKind, BoxNode};
use super::super::wrap_items::wrap_items;
use super::super::wrap_mixed::wrap_mixed;

/* An inline pseudo-element is its text as one run in its own style. One
 * laid out as a box of its own (block, inline-block, flex, grid, out of
 * flow, or an item of a flex or grid host, which blockifies it: the dots,
 * rules and overlays pages draw with content: "") is that box, with its
 * background image, around its text; empty content leaves it childless,
 * sized by its width, height and padding. An empty inline one draws
 * nothing. `at` is the host element and whether it blockifies. */
pub(super) fn pseudo_box(
    p: &PseudoText,
    text: &str,
    link: &Option<String>,
    at: (usize, bool),
) -> Option<BoxNode> {
    let (id, blockify) = at;
    let mut style = p.style;
    style.is_block |= blockify;
    let kind = box_kind(&style);
    if matches!(kind, BoxKind::Inline) {
        if text.is_empty() {
            return None;
        }
        let mut run = leaf(BoxKind::Text(String::from(text)), &style, link, id);
        run.style = style;
        return Some(run);
    }
    let mut kids = Vec::new();
    if !text.is_empty() {
        kids.push(leaf(BoxKind::Text(String::from(text)), &style, link, id));
    }
    let kids = match kind {
        BoxKind::Flex | BoxKind::Grid => wrap_items(&style, kids),
        _ => wrap_mixed(&style, kids),
    };
    let mut node = leaf(kind, &style, link, id);
    (node.style, node.bg_image, node.children) = (style, p.bg_image.clone(), kids);
    Some(node)
}
