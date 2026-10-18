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

//! What the shell does once a tick, on the uptime clock: the clock read
//! again, subscriptions that failed retried, the desktop followed when the
//! store moved, the policy store's prompts checked while it is awake, dead
//! windows and overdue launches said, and the dock's pulses expired.

use super::refresh_clock::refresh_clock;
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::Context;

pub(super) fn on_tick(ctx: &mut Context, tick_ms: i64) {
    refresh_clock(ctx);
    if !ctx.links.input_ready {
        let port = ctx.ports.input_router_port;
        crate::setup::subscribe_input(ctx, port);
    }
    if !ctx.links.wm_notify_ready {
        let port = ctx.ports.wm_port;
        crate::setup::subscribe_wm(ctx, port);
    }
    /*
     * Keep the desktop icons in step with the filesystem, but only
     * when the store says something moved. Both listings below walk a
     * directory and copy it across an IPC boundary, and they used to do
     * that every second whether or not there was anything to find.
     */
    if crate::server::store_changed::since_last_look() {
        if crate::server::desktop::refresh(ctx) {
            crate::server::repaint::repaint(ctx);
        }
        crate::server::packages::refresh(ctx);
    }
    crate::server::wallpaper_policy::retry(ctx);
    crate::server::installed_apps::load_once(ctx);
    crate::server::store_health::check(ctx);
    // Asks the policy store, so not while it is quiet (refresh_clock).
    if ctx.readings.policy_gap.due(tick_ms.max(0) as u64) {
        crate::server::handlers::live_prompt::check(ctx);
    }
    // A window whose process ended without its close reaching the
    // shell: the dock mark and the menubar's app name go with it.
    if crate::state::forget_dead_windows(&mut ctx.taskbar, |p| nonos_libc::mk_pid_alive(p)) != 0 {
        crate::server::repaint::repaint(ctx);
    }
    // A launch whose window never came is said, not left as a dock
    // that did nothing.
    let dock_now = crate::server::dock_clock::now();
    crate::server::launch_overdue::say(ctx, dock_now);
    let pulse_dirty = crate::state::expire_taskbar_pulses(&mut ctx.taskbar, dock_now);
    crate::state::expire_taskbar_visibility(&mut ctx.taskbar, dock_now);
    if pulse_dirty && ctx.taskbar.visible {
        refresh_taskbar(ctx);
    }
    crate::server::dock_sync::sync(ctx);
}
