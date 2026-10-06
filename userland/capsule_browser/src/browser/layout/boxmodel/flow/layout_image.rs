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

use super::super::ctx::Ctx;
use super::super::display_list::{Content, DisplayList, Fragment};
use super::super::edges_x::edges_x;
use super::super::edges_y::edges_y;
use super::super::replaced_size::replaced_size;
use super::super::tree::{BoxKind, BoxNode};

/// Lay an image as a box of its own at (x, y) within `avail` px: its size
/// from the pinned border box when its insets fixed one, else from its
/// replaced size, with its padding and border around the picture. Returns
/// the border-box height.
pub(crate) fn layout_image(
    n: &BoxNode,
    x: i32,
    y: i32,
    avail: i32,
    frags: &mut DisplayList,
    ctx: Ctx,
) -> i32 {
    let BoxKind::Image { src, alt } = &n.kind else { return 0 };
    let s = &n.style;
    let ((el, er), (et, eb)) = (edges_x(s), edges_y(s));
    let (w, h) = match ctx.pin {
        Some(p) => {
            let inner = (p.w - el - er).max(0);
            let h = p.h.unwrap_or_else(|| replaced_size(n, inner, ctx.cb.h).1 + et + eb);
            (p.w, h)
        }
        None => {
            let (iw, ih) = replaced_size(n, (avail - el - er).max(0), ctx.cb.h);
            (iw + el + er, ih + et + eb)
        }
    };
    let x = if s.margin_auto_x && w < avail { x + (avail - w) / 2 } else { x };
    let mut f = Fragment::of_box(n, [x, y, w, h], &ctx);
    f.content = Content::Image { src: src.clone(), alt: alt.clone(), fit: s.object_fit };
    frags.push(f);
    h
}
