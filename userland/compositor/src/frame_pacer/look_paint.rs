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

//! Each output's look over the composed rectangle: mirrored, output 0's
//! over the whole canvas (every mirror shows output 0's screen); extended,
//! each output's over its own slice.

use crate::state::damage::Rect;
use crate::state::Context;
use crate::sw_blitter::Surface;

pub fn paint_looks(ctx: &Context, dst: Surface, rect: Rect) {
    let Some(p) = ctx.plan else {
        if let Some(l) = ctx.looks.looks[0].as_deref() {
            super::look::apply(dst, rect, l);
        }
        return;
    };
    for (s, look) in p.slices.iter().zip(ctx.looks.looks.iter()).take(p.n) {
        let Some(l) = look.as_deref() else { continue };
        let x0 = rect.x.max(s.x);
        let x1 = rect.x.saturating_add(rect.width).min(s.x + s.w);
        if x0 < x1 {
            super::look::apply(
                dst,
                Rect { x: x0, y: rect.y, width: x1 - x0, height: rect.height },
                l,
            );
        }
    }
}
