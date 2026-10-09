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

use super::compose::{compose, Scene};
use crate::state::attach_kernel::Kernel;
use crate::state::damage::Rect;
use crate::state::Context;
use crate::sw_blitter::Surface;

pub fn paint(ctx: &mut Context, rect: Rect) {
    let dst = Surface {
        base_va: ctx.backing_va,
        stride: ctx.stride,
        width: ctx.width,
        height: ctx.height,
        byte_len: ctx.backing_len,
    };
    let cursor = ctx.cursor.current();
    let scene = Scene {
        scene: &mut ctx.scene,
        attach: &mut ctx.attach,
        kernel: &mut Kernel,
        damage: &mut ctx.damage,
    };
    compose(dst, rect, scene, cursor.visible.then_some((cursor.x, cursor.y)));
}
