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
//! after every repaint and on the clock's tick: placed in the new band
//! first, then taken from the old, so the compositor never shows the chrome
//! in neither. A place not answered taken leaves the chrome where it was
//! and the other band owed a removal, as the compositor may still take it;
//! the next sync places it again or, with the prompt gone, takes it back.
//! A place taken puts it in the new band whatever the removal then gets; a
//! removal left unanswered is asked again on each sync until the compositor
//! answers it.

use crate::compositor_client::{push_scene_submit, scene_remove_band::remove_band};
use crate::state::trusted_rule::{in_band, moves, Band};
use crate::state::Context;

fn prompt_up(ctx: &Context) -> bool {
    ctx.pending.pending_consent.is_some()
        || ctx.pending.pending_pkg_install.is_some()
        || ctx.pending.live_prompt.showing()
        || ctx.pending.pending_delete.showing()
        || ctx.trusted.power_off.showing()
        || ctx.trusted.input_menu.showing()
        || ctx.lock.in_band()
}

pub fn sync(ctx: &mut Context) {
    let want = in_band(prompt_up(ctx), ctx.trusted.attention);
    let handle = ctx.chrome.shown_handle();
    if handle == 0 {
        return;
    }
    let port = ctx.ports.compositor_port;
    if let Some((place, _)) = moves(ctx.trusted.band.up, want) {
        let rid = ctx.issue_request_id();
        let (w, h) = (ctx.width, ctx.height);
        if push_scene_submit(port, rid, handle, (0, 0, w, h), place).is_err() {
            ctx.trusted.band = ctx.trusted.band.place_unsure();
            return;
        }
        ctx.trusted.band = Band::placed(want);
    }
    let Some(take) = ctx.trusted.band.take() else { return };
    let rid = ctx.issue_request_id();
    ctx.trusted.band = ctx.trusted.band.taken(remove_band(port, rid, take));
}
