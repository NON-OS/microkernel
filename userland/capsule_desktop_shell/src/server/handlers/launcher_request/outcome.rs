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

//! A launch asked of the kernel: a new window spawned, or the running one
//! raised.

use core::sync::atomic::Ordering;

use nonos_libc::mk_spawn_instance;

use super::focus::focus_service;
use super::refusal::{is_single_instance, REFUSED};
use crate::state::apps::LauncherApp;
use crate::state::says::{NOTHING_REFUSED, NOT_ASKED};

/// What a dock-launch click actually did, so the caller can surface it on
/// screen. `Queued` means the kernel accepted a new-window spawn; `Focused`
/// means the slot table was full and the running window was raised instead.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LaunchOutcome {
    Queued,
    Focused,
    Failed,
}

// Clicking a dock app asks the kernel to spawn another attested instance.
// The kernel queues the request and init performs the spawn in its own
// context, because doing it inline in this shell's syscall corrupted the
// caller. A click with a free instance slot returns ok and a new window
// appears a tick later; when every declared slot is live the kernel returns
// an error and we focus the running instance instead, so a click is never a
// dead end. Single-instance services skip the spawn and always focus.
pub fn request(app: &LauncherApp) -> LaunchOutcome {
    request_service(app.service)
}

/// Launch, or focus if already running, whatever capsule owns `service`. Used
/// by both the dock (a desktop app) and the Launchpad (an installed tool).
pub fn request_service(service: &[u8]) -> LaunchOutcome {
    let rc = if is_single_instance(service) { NOT_ASKED } else { mk_spawn_instance(service) };
    if rc >= 0 {
        return LaunchOutcome::Queued;
    }
    let outcome = focus_service(service);
    /* Kept only for the failure it explains, which says it next. */
    REFUSED.store(
        if outcome == LaunchOutcome::Failed { rc } else { NOTHING_REFUSED },
        Ordering::Relaxed,
    );
    outcome
}
