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

use crate::browser::css::{Computed, Size};

use super::ctx::Ctx;

impl Ctx {
    /// The context a box at flow y `y` with style `s` lays itself out in: a
    /// z-index opens a stacking level, a fixed box pins its subtree to the
    /// viewport on scroll, opacity multiplies down, and a sticky box anchors
    /// its subtree so paint shifts the whole of it once scrolled past.
    pub(crate) fn enter(self, s: &Computed, y: i32) -> Ctx {
        let mut ctx = self;
        if s.z != 0 {
            ctx.z = s.z;
        }
        ctx.fixed |= s.is_fixed;
        if s.opacity != 255 {
            ctx.alpha = ((ctx.alpha as u16 * s.opacity as u16) / 255) as u8;
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
