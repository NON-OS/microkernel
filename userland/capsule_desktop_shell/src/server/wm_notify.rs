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

use crate::state::wm_notice::{decode_wm_notice, WmEvent};
use crate::state::{focus_moved, set_full_screen, Context, TASKBAR_WINDOW_ID, TOAST_WINDOW_ID};

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
    super::wm_notify_switch::note(ctx, owner_pid, window_id, notice.event);
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
        WmEvent::Focused => {
            let app = super::handlers::instances::app_of_pid(owner_pid);
            if focus_moved(&mut ctx.taskbar, app) {
                super::repaint::repaint(ctx);
            }
            return true;
        }
        WmEvent::Opened => {}
    }
    // The dock's window is never raised: the shell draws the dock in its
    // chrome band, over every application window, and the window manager
    // ranks the shell's popup windows over every other window whatever their
    // z, so what is drawn on top and what a press there reaches stay one.
    let opened = notice.event == WmEvent::Opened;
    super::wm_notify_track::track(ctx, owner_pid, window_id, opened);
    true
}
