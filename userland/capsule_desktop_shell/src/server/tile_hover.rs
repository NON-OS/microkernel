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
//! The pointer over the dock's tiles and the Launchpad's page tabs. Each
//! change redraws only when it changes what is lit: the Launchpad through
//! the input repaint, the dock through its own refresh.

use crate::render::dock_lit;
use crate::render::launchpad::tabs_lit;
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::Context;

type Lit = ((Option<usize>, Option<usize>), Option<usize>);

fn lit(ctx: &Context) -> Lit {
    (dock_lit(ctx), if ctx.launchpad { tabs_lit(ctx) } else { None })
}

fn redraw(ctx: &mut Context, was: Lit) {
    let now = lit(ctx);
    if now.1 != was.1 {
        super::repaint::repaint(ctx);
    } else if now.0 != was.0 && ctx.taskbar.visible {
        refresh_taskbar(ctx);
    }
}

pub fn moved(ctx: &mut Context, x: i32, y: i32) {
    let was = lit(ctx);
    ctx.tile_pointer.moved(x, y);
    redraw(ctx, was);
}

pub fn pressed(ctx: &mut Context, x: i32, y: i32, touch: bool) {
    if touch {
        ctx.tile_pointer.touched(x, y);
    } else {
        ctx.tile_pointer.pressed(x, y);
    }
}

pub fn released(ctx: &mut Context) {
    let was = lit(ctx);
    ctx.tile_pointer.released();
    redraw(ctx, was);
}
