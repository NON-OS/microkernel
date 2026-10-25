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

//! A running app's window raised: its newest window, or any live instance
//! of a service.

use nonos_libc::{mk_ipc_send_to_pid, mk_pid_alive};

use super::frame::focus_frame;
use super::outcome::LaunchOutcome;
use crate::state::{raise_tracked, reach_by, Context, Reach};

/// Focus whatever already owns `service`, spawning nothing.
///
/// The dock uses this when it knows the app is running. Going through
/// `request_service` spawned a fresh window on every click and left a
/// minimized one hidden, which made it unreachable: this message is the only
/// thing that reaches `wm::window_restore`.
///
/// Any live instance will do: with the first window closed, the base name
/// is gone from the registry while "app.terminal.2" still holds a window.
pub fn focus_service(service: &[u8]) -> LaunchOutcome {
    if super::super::instances::each_live_pid(service, focus_pid) {
        LaunchOutcome::Focused
    } else {
        LaunchOutcome::Failed
    }
}

/// Raise app `index`'s newest window the window manager said is open,
/// restoring it if minimised. A window whose process is gone is forgotten
/// on the way (state/taskbar/route.rs). Failed when the app has no window
/// left: the caller opens one, rather than handing the click to an instance
/// with no window to show.
pub fn focus_app(ctx: &mut Context, index: usize) -> LaunchOutcome {
    match raise_tracked(&mut ctx.taskbar, index, reach_pid) {
        Some(_) => LaunchOutcome::Focused,
        None => LaunchOutcome::Failed,
    }
}

/// Send `pid` the focus frame, and say whether its process took it, is
/// busy, or is gone. A process that has ended is gone, though the kernel
/// would still take a frame into its inbox (state/taskbar/route.rs).
fn reach_pid(pid: u32) -> Reach {
    reach_by(pid, |p| mk_pid_alive(p), send_focus)
}

/// Send `pid` the focus frame; false when it is gone or busy.
pub(crate) fn focus_pid(pid: u32) -> bool {
    reach_pid(pid) == Reach::Taken
}

fn send_focus(pid: u32) -> i64 {
    let frame = focus_frame();
    mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len())
}
