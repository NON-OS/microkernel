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

//! Hand a path to an app, or a command line to the Terminal: leave it under
//! the app's name for its next OP_TAKE_OPEN_ARG, and bring up the window that
//! will take it.
//!
//! A running app's window asks for a path on its ticks (the editor and Files
//! every tick, Music, Video and the image viewer about once a second, the
//! Terminal twice a second), so it takes it within a second. Launching another
//! instance on top of it used to open an empty window while the window
//! already open took the file; so an app with a window open has that window
//! raised (restored if minimised) to show it, and only an app with none is
//! launched.

use alloc::string::String;

use nonos_libc::mk_uptime_ms;

use super::launcher_request::{self, LaunchOutcome};
use crate::state::open_arg::{PendingCommand, TERMINAL};
use crate::state::{Context, LAUNCHER_APPS};

pub fn hand_over(ctx: &mut Context, index: usize, path: String) -> LaunchOutcome {
    let Some(app) = LAUNCHER_APPS.get(index) else { return LaunchOutcome::Failed };
    let Ok(service) = core::str::from_utf8(app.service) else { return LaunchOutcome::Failed };
    ctx.pending_open.insert(String::from(service), path);
    let outcome = bring_up(ctx, index);
    if outcome == LaunchOutcome::Failed {
        // Nothing will take the path now; left behind, the next window of
        // the app would open a file nobody asked it for.
        ctx.pending_open.remove(service);
    }
    outcome
}

/// Leave `line` (state::open_arg's `run:` or `type:` form) for the Terminal.
/// A newer tile click replaces an older command not yet taken.
pub fn hand_command(ctx: &mut Context, line: String) -> LaunchOutcome {
    let Some(index) = LAUNCHER_APPS.iter().position(|a| a.service == TERMINAL) else {
        return LaunchOutcome::Failed;
    };
    ctx.pending_command = Some(PendingCommand { line, at_ms: mk_uptime_ms() });
    let outcome = bring_up(ctx, index);
    if outcome == LaunchOutcome::Failed {
        ctx.pending_command = None;
    }
    outcome
}

fn bring_up(ctx: &mut Context, index: usize) -> LaunchOutcome {
    if launcher_request::focus_app(ctx, index) == LaunchOutcome::Focused {
        return LaunchOutcome::Focused;
    }
    let Some(app) = LAUNCHER_APPS.get(index) else { return LaunchOutcome::Failed };
    let outcome = crate::apps_off::request(app);
    crate::apps_off::expect(ctx, app.service, outcome, crate::server::dock_clock::now());
    outcome
}
