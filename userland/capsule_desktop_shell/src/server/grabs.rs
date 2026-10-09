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

use crate::state::grab_rule::{steps, wanted, Modal};
use crate::state::Context;

/// Bring the router's grabs in line with what is up on the desktop. Called
/// after every batch of messages, and right where a drag or an edit starts or
/// ends, so a press never lands between the state and its grab.
pub fn sync(ctx: &mut Context) {
    let want = wanted(Modal {
        drag: ctx.drag_from.is_some(),
        launchpad: ctx.launchpad,
        menu: ctx.desktop_menu.is_some() || ctx.menubar.open.is_some(),
        dialog: ctx.pending_consent.is_some()
            || ctx.pending_pkg_install.is_some()
            || ctx.live_prompt.showing()
            || ctx.pending_delete.showing(),
        rename: ctx.rename.is_some(),
    });
    let (release, request) = steps(ctx.grab_held, want);
    if release {
        let rid = ctx.issue_request_id();
        let _ = crate::input_router_client::release_grab(ctx.input_router_port, rid);
        ctx.grab_held = 0;
    }
    if let Some(mask) = request {
        let rid = ctx.issue_request_id();
        // Refused only while another holder (the installer, the boot splash)
        // has the whole screen; nothing is held then, and the next sync asks
        // again.
        if crate::input_router_client::grab(ctx.input_router_port, rid, mask).is_ok() {
            ctx.grab_held = mask;
        }
    }
}
