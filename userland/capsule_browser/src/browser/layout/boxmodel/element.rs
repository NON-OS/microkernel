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

mod replaced;
mod split_inline;
mod split_pieces;

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::element_box::element_box;
use super::tree::BoxNode;
use super::walk::{ElementIn, Walk};
use replaced::replaced;
use split_inline::push_split;

/* Per-tag dispatch for one element child: non-rendered subtrees drop,
 * <br>, <img> and form fields are leaves, everything else recurses. */
pub(super) fn element(
    w: &mut Walk,
    item: &ElementIn,
    parent: &Computed,
    link: &Option<String>,
    depth: u32,
    out: &mut Vec<BoxNode>,
) {
    let styles = w.styles;
    let style = &styles[item.ch];
    if style.display_none {
        return;
    }
    let tag = item.c.tag.as_str();
    /* Never rendered, whatever display an author gives them. [hidden], a
     * closed <dialog>, an unopened popover and, by the <noscript>
     * policy, <noscript> are display:none in the cascade instead, so an
     * author display (or the policy) can show them. */
    if matches!(tag, "script" | "style" | "head" | "title" | "template") {
        return;
    }
    *w.count += 1;
    if !replaced(w, item, parent, link, style, out) {
        let in_items = parent.is_flex || parent.is_grid;
        push_split(element_box(w, item, *style, link, depth), in_items, out);
    }
}
