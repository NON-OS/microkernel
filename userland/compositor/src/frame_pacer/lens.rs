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

//! Each magnified output's lens for this frame, from the point it follows;
//! a lens that moved has the whole of its output's view drawn again.
//! Mirrored, only output 0 has a view of its own; extended, every output.

use crate::display::access_rule::FOLLOW_FOCUS;
use crate::display::magnifier_rule::lens;
use crate::state::damage::Rect;
use crate::state::Context;

pub fn refresh(ctx: &mut Context) {
    let n = ctx.plan.map_or(1, |p| p.n);
    for i in 0..ctx.looks.lens.len() {
        let a = ctx.looks.access[i];
        let Some((x, w, h, screen)) = view(ctx, i).filter(|_| i < n && a.magnified()) else {
            ctx.looks.lens[i] = None;
            continue;
        };
        let (fx, fy) = if a.follow == FOLLOW_FOCUS { focus_centre(ctx) } else { pointer(ctx) };
        let l = lens((fx.saturating_sub(x).min(w), fy.min(h)), (w, h), screen, a.magnify);
        if ctx.looks.lens[i] != Some(l) {
            ctx.looks.lens[i] = Some(l);
            ctx.damage.accumulate(Rect { x, y: 0, width: w, height: h });
        }
    }
}

/// Output `i`'s slice of the canvas (x, width, height) and its screen size.
fn view(ctx: &Context, i: usize) -> Option<(u32, u32, u32, (u32, u32))> {
    let Some(p) = ctx.plan else {
        return Some((0, ctx.width, ctx.height, (ctx.screen.width, ctx.screen.height)));
    };
    let s = p.slices[i];
    let screen = if i == 0 { ctx.screen } else { ctx.heads.get(i - 1)?.screen };
    Some((s.x, s.w, s.h, (screen.width, screen.height)))
}

fn pointer(ctx: &Context) -> (u32, u32) {
    let c = ctx.cursor.current();
    (c.x, c.y)
}

/// The focused window's centre, or the pointer when no window has focus.
fn focus_centre(ctx: &Context) -> (u32, u32) {
    let pid = ctx.focus.focused();
    let top = ctx.scene.layers().filter(|l| pid != 0 && l.owner_pid == pid).max_by_key(|l| l.z);
    top.map_or_else(|| pointer(ctx), |l| (l.x + l.width / 2, l.y + l.height / 2))
}
