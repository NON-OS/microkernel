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

use alloc::string::{String, ToString};

use crate::browser::css::Computed;

use super::box_kind::box_kind;
use super::collect::collect;
use super::grid_place::resolve_grid_places;
use super::leaf::leaf;
use super::list_marker::add_marker;
use super::pseudo_box::add_pseudos;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};
use super::wrap_items::wrap_items;
use super::wrap_mixed::wrap_mixed;

/* Generic element: recurse into its children and pick a formatting context.
 * Anchors thread their href down so links survive layout. */
pub(super) fn element_box(
    w: &mut Walk,
    item: &ElementIn,
    style: Computed,
    link: &Option<String>,
    depth: u32,
) -> BoxNode {
    let tag = item.c.tag.as_str();
    let link = if tag == "a" {
        item.c.attr("href").map(|h| h.to_string()).or_else(|| link.clone())
    } else {
        link.clone()
    };
    let (dom, styles, bgs, grids, pseudos) = (w.dom, w.styles, w.bg_images, w.grids, w.pseudos);
    let mut kids =
        collect(dom, item.ch, &style, styles, bgs, grids, pseudos, &link, depth + 1, w.count);
    /* An edited textarea renders its value, which typing keeps current. */
    if tag == "textarea" {
        if let Some(v) = item.c.attr("value") {
            let v = v.to_string();
            kids.clear();
            kids.push(leaf(BoxKind::Text(v), &style, &None, item.ch));
        }
    }
    add_pseudos(w, item.ch, &link, &mut kids);
    add_marker(w, item, &style, &mut kids);
    let kind = box_kind(&style);
    let mut kids = match kind {
        BoxKind::Flex | BoxKind::Grid => wrap_items(&style, kids),
        /* An inline-block runs a block context inside: wrap it like a block. */
        BoxKind::Block | BoxKind::InlineBlock => wrap_mixed(&style, kids),
        _ => kids,
    };
    /* Grid items' named or numeric positions resolve while names are known. */
    if matches!(kind, BoxKind::Grid) {
        resolve_grid_places(w, item.ch, &style, &mut kids);
    }
    let bg_image = w.bg_images.get(item.ch).cloned().flatten();
    let mut node = leaf(kind, &style, &link, item.ch);
    (node.style, node.bg_image, node.children) = (style, bg_image, kids);
    node
}
