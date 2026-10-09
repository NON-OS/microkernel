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

use crate::browser::css::{Computed, Position, Size};

use super::ctx::Ctx;

impl Ctx {
    /// The context the box at tree position `seq` and flow y `y`, with
    /// style `s`, lays itself out in: a z-index opens a stacking context (as
    /// opacity or a transform do, at level 0; CSS Color 3 3.2, CSS
    /// Transforms 1), a positioned box paints in its context's positioned
    /// layer, a fixed box pins its subtree to the viewport on scroll,
    /// opacity multiplies down, and a sticky box anchors its subtree so
    /// paint shifts the whole of it once scrolled past.
    pub(crate) fn enter(self, s: &Computed, y: i32, seq: usize) -> Ctx {
        let mut ctx = self;
        let alpha = s.alpha();
        let own = alpha != 255 || s.fx.transform.is_some();
        let z = s.z_index().or(own.then_some(0));
        ctx.z = ctx.z.enter(z, s.position != Position::Static || s.is_sticky, seq);
        ctx.fixed |= s.is_fixed;
        if alpha != 255 {
            ctx.alpha = ((ctx.alpha as u16 * alpha as u16) / 255) as u8;
        }
        if s.is_sticky && ctx.sticky.is_none() {
            let top = match s.top {
                Size::Px(p) => p as i32,
                _ => 0,
            };
            ctx.sticky = Some((y, top));
        }
        ctx
    }
}
