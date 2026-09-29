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

use super::super::tree::BoxNode;
use super::ctx::Ctx;
use super::fragment::{Content, Fragment};
use super::radii::radii;

impl Fragment {
    /// A word or an image on a line: content with no box decoration of its
    /// own beyond an inline background.
    pub(crate) fn leaf(
        r: [i32; 4],
        bg: u32,
        content: Content,
        href: Option<String>,
        node: usize,
        ctx: &Ctx,
    ) -> Fragment {
        let [x, y, w, h] = r;
        Fragment {
            x,
            y,
            w,
            h,
            bg,
            href,
            content,
            z: ctx.z.key,
            clip: ctx.clip,
            clip_r: ctx.clip_r,
            fixed: ctx.fixed,
            sticky: ctx.sticky,
            alpha: ctx.alpha,
            node,
            ..Fragment::BLANK
        }
    }

    /// A box's own rectangle at `r` ([x, y, w, h]): its background, border,
    /// corner radii and shadow, painted under its content.
    pub(crate) fn of_box(node: &BoxNode, r: [i32; 4], ctx: &Ctx) -> Fragment {
        let s = &node.style;
        let mut f = Fragment::leaf(r, s.bg, Content::None, node.href.clone(), node.dom_id, ctx);
        f.border = [s.border_top, s.border_right, s.border_bottom, s.border_left];
        f.border_color = if s.border_color != 0 { s.border_color } else { s.color };
        (f.bg_image, f.mask) = (node.bg_image.clone(), s.fx.mask && node.bg_image.is_some());
        (f.bg_size, f.bg_repeat, f.shadow) = (s.bg_size, s.bg_repeat, s.shadow);
        (f.radius, f.z) = (radii(s, r[2]), ctx.z.decor());
        f
    }
}
