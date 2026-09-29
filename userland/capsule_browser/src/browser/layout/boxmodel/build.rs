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

use crate::browser::css::{Computed, GridSpec, PseudoText};
use crate::browser::dom::Dom;

use super::collect::collect;
use super::geom::body_id::{body_id, root_id};
use super::post::propagate::{fill_natural, propagate};
use super::tree::{BoxKind, BoxNode};
use super::walk::Walk;
use super::wrap_mixed::wrap_mixed;

/// An image's natural size (width, height) in px by its src, when known.
pub type Natural<'a> = &'a dyn Fn(&str) -> Option<(u32, u32)>;

/* Box tree for the page: rooted at the root element (<html>, or <body> or
 * the document when a page has none), display:none subtrees dropped, mixed
 * children wrapped. Per-node background images and grid specs travel
 * alongside the styles for the collect walk. body's background and
 * overflow pass to the root and the viewport as CSS prescribes, and each
 * image learns its natural size from `natural`. */
pub fn build(
    dom: &Dom,
    styles: &[Computed],
    bg_images: &[Option<String>],
    grids: &[Option<GridSpec>],
    pseudos: &[(Option<PseudoText>, Option<PseudoText>)],
    natural: Natural<'_>,
) -> BoxNode {
    let root_id = root_id(dom);
    let style = styles.get(root_id).copied().unwrap_or_else(Computed::root);
    let mut count = 0usize;
    let mut w = Walk { dom, styles, bg_images, grids, pseudos, count: &mut count };
    let children = collect(&mut w, root_id, &style, &None, 0);
    let mut root = BoxNode {
        kind: BoxKind::Block,
        style,
        href: None,
        dom_id: root_id,
        bg_image: bg_images.get(root_id).cloned().flatten(),
        grid_place: None,
        children: wrap_mixed(&style, children),
        aux: Default::default(),
    };
    propagate(&mut root, body_id(dom));
    fill_natural(&mut root, natural, 0);
    root
}
