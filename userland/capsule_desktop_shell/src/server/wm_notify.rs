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

use super::handlers::instances::app_of_pid;
use super::refresh_taskbar::refresh_taskbar;
use super::wm_notify_toast::toast_window_event;
use crate::state::wm_notice::{decode_wm_notice, WmEvent};
use crate::state::{
    set_full_screen, track_window_closed, track_window_opened, Context, TASKBAR_WINDOW_ID,
    TOAST_WINDOW_ID,
};

pub fn handle(ctx: &mut Context, buf: &[u8]) -> bool {
    let Some(notice) = decode_wm_notice(buf) else {
        return false;
    };
    let Some(notice) = notice else {
        return true;
    };
    let (owner_pid, window_id) = (notice.owner_pid, notice.window_id);
    if window_id == TASKBAR_WINDOW_ID || window_id == TOAST_WINDOW_ID {
        return true;
    }
    // A full-screen window shown or gone hides or brings back the dock, for
    // any app's window, in the launcher table or not; the dock's sync after
    // this batch draws it and opens or closes its window.
    match notice.event {
        WmEvent::FullScreen(on) => {
            set_full_screen(&mut ctx.taskbar, owner_pid, window_id, on);
            return true;
        }
        WmEvent::Closed => {
            set_full_screen(&mut ctx.taskbar, owner_pid, window_id, false);
        }
        WmEvent::Opened => {}
    }
    // The dock's window is never raised: the shell draws the dock in its
    // chrome band, over every application window, and the window manager
    // ranks the shell's popup windows over every other window whatever their
    // z, so what is drawn on top and what a press there reaches stay one.
    let opened = notice.event == WmEvent::Opened;
    let named = ctx.taskbar.active;
    // Any instance of an app counts as that app ("app.terminal.2" is the
    // Terminal), so the dock marks it, the menubar names it and the toast
    // says which app opened. A close is matched by window id to the open the
    // shell saw: the closing process may already be out of the registry.
    let app = if opened {
        let app = app_of_pid(owner_pid);
        if let Some(index) = app {
            track_window_opened(&mut ctx.taskbar, window_id, owner_pid, index);
        }
        app
    } else {
        track_window_closed(&mut ctx.taskbar, owner_pid, window_id)
    };
    if let Some(index) = app {
        // The menubar names the active app. Only the dock was presented here,
        // so after a close the bar went on naming the app that had gone until
        // something else repainted it.
        if ctx.taskbar.active != named {
            super::repaint::repaint(ctx);
        } else {
            refresh_taskbar(ctx);
        }
        toast_window_event(ctx, opened, index);
    }
    true
}
