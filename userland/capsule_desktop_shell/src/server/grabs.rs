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

use crate::state::grab_rule::{after_release, after_request, steps, wanted, Modal};
use crate::state::Context;

/// Bring the router's grabs in line with what is up on the desktop. Called
/// after every batch of messages, and right where a drag or an edit starts or
/// ends, so a press never lands between the state and its grab.
pub fn sync(ctx: &mut Context) {
    let want = wanted(Modal {
        drag: ctx.desk_ui.drag_from.is_some(),
        launchpad: ctx.launchpad_ui.launchpad,
        menu: ctx.desk_ui.desktop_menu.is_some() || ctx.menubar.open.is_some(),
        dialog: ctx.pending.pending_consent.is_some()
            || ctx.pending.pending_pkg_install.is_some()
            || ctx.pending.live_prompt.showing()
            || ctx.pending.pending_delete.showing()
            || ctx.trusted.attention != crate::state::attention::Attention::None
            || ctx.lock.in_band(),
        rename: ctx.desk_ui.rename.is_some(),
    });
    let (release, request) = steps(ctx.links.grab_held, want);
    let port = ctx.ports.input_router_port;
    if release {
        let rid = ctx.issue_request_id();
        let answer = crate::input_router_client::release_grab(port, rid);
        ctx.links.grab_held = after_release(ctx.links.grab_held, answer);
        // Still held: the next pass releases it again before asking anew.
        if ctx.links.grab_held.mask != 0 {
            return;
        }
    }
    if let Some(mask) = request {
        let rid = ctx.issue_request_id();
        // Refused only while another holder (the installer, the boot splash)
        // has the whole screen; nothing is held then, and the next sync asks
        // again. Unanswered, it may be held, and is counted so.
        let answer = crate::input_router_client::grab(port, rid, mask);
        ctx.links.grab_held = after_request(ctx.links.grab_held, mask, answer);
    }
}
