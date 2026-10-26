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

//! Keeping the chrome in the band it belongs in (`state/trusted_rule.rs`),
//! after every repaint: placed in the new band first, then taken from the
//! old, so the compositor never shows the chrome in neither. A refusal
//! leaves `up` as it was, and the next repaint tries again.

use crate::compositor_client::{push_scene_submit, scene_remove_band::remove_band, Placement};
use crate::state::trusted_rule::{in_band, moves};
use crate::state::Context;

fn prompt_up(ctx: &Context) -> bool {
    ctx.pending.pending_consent.is_some()
        || ctx.pending.pending_pkg_install.is_some()
        || ctx.pending.live_prompt.showing()
        || ctx.pending.pending_delete.showing()
        || ctx.trusted.power_off.showing()
        || ctx.trusted.input_menu.showing()
}

pub fn sync(ctx: &mut Context) {
    let t = ctx.trusted;
    let want = in_band(prompt_up(ctx), t.attention);
    let Some((place, take)) = moves(t.up, want).filter(|_| t.chrome_handle != 0) else {
        return;
    };
    let rid = ctx.issue_request_id();
    let (port, handle, w, h) = (ctx.ports.compositor_port, t.chrome_handle, ctx.width, ctx.height);
    let at = Placement { x: 0, y: 0, width: w, height: h, z: place };
    if push_scene_submit(port, rid, handle, &at).is_err() {
        return;
    }
    let rid = ctx.issue_request_id();
    let _ = remove_band(port, rid, take);
    ctx.trusted.up = want;
}
