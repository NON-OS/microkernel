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

use alloc::vec::Vec;

use super::ctx::Ctx;
use super::display_list::{BoxDocument, DisplayList};
use super::geom::containing::Containing;
use super::geom::margins::margins;
use super::geom::stack::Stack;
use super::layout_box::layout_box;
use super::post::canvas::canvas;
use super::post::place_out::place_root;
use super::post::unsized_imgs::unsized_imgs;
use super::tree::BoxNode;

/* Lay the whole page: the root element's box against the viewport, (width,
 * height) in px, which is also the initial containing block and the block
 * fixed boxes pin to. The root's background becomes the canvas. Fragments
 * sort by stacking z so the painter and hit-testing agree on order. */
pub fn layout(root: &BoxNode, viewport: (u32, u32)) -> BoxDocument {
    let vp = (viewport.0 as i32, viewport.1 as i32);
    let mut frags: DisplayList = Vec::new();
    let [mt, mr, mb, ml] = margins(&root.style, vp.0);
    let ctx = Ctx {
        cb: Containing { w: vp.0, h: Some(vp.1) },
        clip: None,
        clip_r: [0; 4],
        z: Stack::ROOT,
        fixed: false,
        sticky: None,
        alpha: 255,
        vp,
        pin: None,
    };
    let h = layout_box(root, ml, mt, (vp.0 - ml - mr).max(0), &mut frags, 0, ctx);
    place_root(root, &mut frags, ctx);
    let (canvas_bg, canvas_bg_image) = canvas(root, &mut frags);
    frags.sort_by_key(|f| f.z);
    /* The page is as tall as the lowest thing that scrolls with it, less
     * what an overflow clip cuts off. */
    let mut bottom = mt + h + mb;
    for f in frags.iter().filter(|f| !f.fixed) {
        let end = f.y.saturating_add(f.h);
        bottom = bottom.max(f.clip.map_or(end, |c| end.min(c[3])));
    }
    let content_h = bottom.max(0) as u32;
    BoxDocument { frags, content_h, canvas_bg, canvas_bg_image, unsized_imgs: unsized_imgs(root) }
}
