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

//! Bring the screen and the window manager in line with the dock's rule
//! (state/taskbar/dock_rule.rs), after every batch of messages and on the
//! clock's tick.
//!
//! Hidden, the dock's area of the chrome is cleared to transparent and the
//! whole area, panel and shadow, is committed, so the compositor draws the
//! full-screen window there; and its popup window is closed, because the
//! window manager ranks the shell's popups over every window and the
//! bottom of the screen would go on taking presses meant for the app. Shown,
//! it is drawn over the window and its popup window opened again, so a
//! press there reaches the dock. A call that fails is tried again on the
//! next sync.

use crate::render::layout::bottom_dock_rect;
use crate::state::{dock_work, Context, TASKBAR_WINDOW_ID};
use crate::wm_client;

use super::refresh_taskbar::refresh_taskbar;

const WINDOW_KIND_POPUP: u32 = 3;

pub fn sync(ctx: &mut Context) {
    let work = dock_work(&ctx.taskbar);
    if work.paint {
        refresh_taskbar(ctx);
    }
    match work.window {
        Some(true) => {
            let r = bottom_dock_rect(ctx.width, ctx.height);
            let rid = ctx.issue_request_id();
            let opened = wm_client::window_open(
                ctx.wm_port,
                rid,
                TASKBAR_WINDOW_ID,
                WINDOW_KIND_POPUP,
                r.x,
                r.y,
                r.width,
                r.height,
            );
            if opened.is_ok() {
                ctx.taskbar.window_open = true;
            }
        }
        Some(false) => {
            let rid = ctx.issue_request_id();
            // Refused is gone already (the window manager holds no such
            // window); only a call that got no answer is tried again.
            match wm_client::window_close(ctx.wm_port, rid, TASKBAR_WINDOW_ID) {
                Ok(()) | Err("wm rejected window_close") => ctx.taskbar.window_open = false,
                Err(_) => {}
            }
        }
        None => {}
    }
}
