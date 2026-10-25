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

//! A dock icon pressed: the app's open window raised, or the app opened,
//! and a toast saying which.

use crate::apps_off::toast_failed;
use crate::server::handlers::launcher_request::{self, LaunchOutcome};
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::apps::LauncherApp;
use crate::state::{mark_taskbar_launch, Context, NotifyLevel};

pub(super) fn launch_at(ctx: &mut Context, index: usize, app: &LauncherApp) {
    // Uptime: the dock's pulse and the window wait never read the
    // wall clock (state/taskbar/expect.rs).
    let now = crate::server::dock_clock::now();
    // A window the app has open is raised (restored if minimised), as
    // a running app's dock icon should; one whose process is gone is
    // forgotten. With none left the app is opened afresh, never
    // handed to an instance with no window to show.
    if launcher_request::focus_app(ctx, index) == LaunchOutcome::Focused {
        mark_taskbar_launch(&mut ctx.taskbar, index, now);
        refresh_taskbar(ctx);
        return;
    }
    let outcome = crate::apps_off::request(app);
    // Surface the result on screen: with no serial port a dock click is
    // otherwise silent, so a toast says which branch it took.
    match outcome {
        LaunchOutcome::Queued => {
            ctx.toasts.push(
                b"opening a new window",
                NotifyLevel::Info,
                crate::server::toast_clock::now(),
            );
            crate::apps_off::say_default(ctx, crate::server::toast_clock::now());
            mark_taskbar_launch(&mut ctx.taskbar, index, now);
        }
        // The kernel spawned nothing and woke an instance of the app
        // instead: its idle base, or the app's one window process.
        LaunchOutcome::Focused => mark_taskbar_launch(&mut ctx.taskbar, index, now),
        LaunchOutcome::Failed => toast_failed(ctx, app.service, crate::server::toast_clock::now()),
    }
    crate::apps_off::expect(ctx, app.service, outcome, now);
    refresh_taskbar(ctx);
}
