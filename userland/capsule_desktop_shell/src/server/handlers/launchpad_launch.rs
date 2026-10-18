// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A Launchpad tile chosen: an app opened, a command-line tool handed to the
//! Terminal, an installed app put to its consent, or a package installed;
//! then the Launchpad closes.

use super::launcher_request::LaunchOutcome;
use super::launchpad::close;
use crate::render::launchpad::Target;
use crate::state::open_arg::{tool_command, TERMINAL};
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};

pub(super) fn launch(ctx: &mut Context, t: Target) {
    match t {
        Target::App(a) => crate::apps_off::open(ctx, LAUNCHER_APPS[a].service),
        Target::Tool(i) => {
            // A tool is a command-line program: it runs in the Terminal,
            // which spawns it as its own job so its output streams into the
            // tab. The tile hands the Terminal the tool's command line, run
            // or typed for its arguments (state::open_arg).
            // A name the table should never hold opens a bare Terminal.
            match tool_command(TOOL_APPS[i].label) {
                Some(line) => {
                    if super::hand_over::hand_command(ctx, line) == LaunchOutcome::Failed {
                        crate::apps_off::toast_failed(
                            ctx,
                            TERMINAL,
                            crate::server::toast_clock::now(),
                        );
                    }
                }
                None => crate::apps_off::open(ctx, TERMINAL),
            }
        }
        Target::Installed(i) => {
            if let Some(name) = ctx.store_apps.installed_apps.get(i).cloned() {
                ctx.pending.pending_consent = Some(name);
            }
        }
        Target::Package(i) => super::pkg_install::begin(ctx, i),
    }
    close(ctx);
}
