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

/* What a launch is followed for: the window it should bring, and the default
 * tier a Qwen window opened on. */

use crate::server::handlers::launcher_request::LaunchOutcome;
use crate::state::toast::UptimeMs;
use crate::state::{expect_window, Context, NotifyLevel, Uptime, LAUNCHER_APPS};

/*
 * Follow a launch that should bring a new window: a spawn init accepted, or
 * an instance woken that has no window yet. Past the wait the shell says the
 * app did not open (server/runner/run.rs). A focus that raised a window the
 * app already had brings none, and Qwen's window is the Linux personality's,
 * never matched to the tile, so neither is followed.
 */
pub fn expect(ctx: &mut Context, service: &[u8], outcome: LaunchOutcome, now: Uptime) {
    if service == super::qwen::SERVICE {
        return;
    }
    let Some(index) = LAUNCHER_APPS.iter().position(|a| a.service == service) else { return };
    let has_window = ctx.taskbar.open.get(index).copied().unwrap_or(false);
    let wants = match outcome {
        LaunchOutcome::Queued => true,
        LaunchOutcome::Focused => !has_window,
        LaunchOutcome::Failed => false,
    };
    if wants {
        expect_window(&mut ctx.taskbar, index, now);
    }
}

/* A Qwen window opened on the default tier, not one chosen: said, never silent. */
pub fn say_default(ctx: &mut Context, now: UptimeMs) {
    if let Some(line) = super::qwen::default_said() {
        ctx.toasts.push(&line, NotifyLevel::Info, now);
    }
}
