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

use super::expect::{expect, say_default};
use super::failed::{qwen_failed, toast_failed};
use super::is_off;
use super::say::say;
use crate::server::handlers::launcher_request::{self, LaunchOutcome};
use crate::state::apps::LauncherApp;
use crate::state::says::OFF_AT_SETUP;
use crate::state::{Context, NotifyLevel};

/* A dock launch, never asked of the kernel for an app it will not spawn. */
pub fn request(app: &LauncherApp) -> LaunchOutcome {
    if is_off(app.service) {
        return LaunchOutcome::Failed;
    }
    if app.service == super::qwen::SERVICE {
        return super::qwen::open();
    }
    launcher_request::request(app)
}

/*
 * Launch `service` from a menu, the Launchpad, a dialog or the escape chord,
 * or say why nothing opened: off at setup, Qwen's own reason, or that the
 * kernel neither spawned a window nor had one to raise.
 */
pub fn open(ctx: &mut Context, service: &[u8]) {
    if is_off(service) {
        say(ctx, service, OFF_AT_SETUP, NotifyLevel::Warn, crate::server::toast_clock::now());
        return;
    }
    if service == super::qwen::SERVICE {
        if super::qwen::open() == LaunchOutcome::Failed {
            qwen_failed(ctx, crate::server::toast_clock::now());
        } else {
            say_default(ctx, crate::server::toast_clock::now());
        }
        return;
    }
    let outcome = launcher_request::request_service(service);
    if outcome == LaunchOutcome::Failed {
        toast_failed(ctx, service, crate::server::toast_clock::now());
    }
    expect(ctx, service, outcome, crate::server::dock_clock::now());
}
