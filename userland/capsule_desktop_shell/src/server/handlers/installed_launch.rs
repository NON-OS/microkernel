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

//! Launch a capsule-store app from the Launchpad. The kernel's spawn path
//! resolves names against its embedded capsules, so it never reaches an app
//! installed at runtime; the installer owns that load. The pid it returns is
//! remembered, so a later click raises the running window, not a second copy.
//! The load takes seconds and this is the shell's frame loop, so the request
//! is only issued here: a quick answer is acted on at once, and a load still
//! running is followed later (`installed_launch_poll.rs`), with a toast.

use super::launch_children::children;
use super::launch_find::{already_running, focus_running};
use super::launch_say::{report_failure, said, toast, ERR_EXIST};
use super::launch_wake::{boot, focus};
use crate::installer_client::{issue_load, Issued};
use crate::state::launch::Launch;
use crate::state::{Context, NotifyLevel};

pub fn launch(ctx: &mut Context, name: &[u8]) {
    if let Some(pid) = ctx.store_apps.installed_pids.get(name).copied() {
        if focus(pid) {
            return;
        }
        ctx.store_apps.installed_pids.remove(name);
    }
    if let Some(pid) = already_running(name) {
        ctx.store_apps.installed_pids.insert(name.to_vec(), pid);
        if focus(pid) {
            return;
        }
        ctx.store_apps.installed_pids.remove(name);
    }
    /*
     * One load at a time: the installer serves them in turn anyway, and each
     * one followed holds places in its reply queue (`state/launch.rs`).
     */
    if let Some(pending) = ctx.store_apps.launch.as_ref() {
        let text = if pending.name == name {
            said(b"Starting ", name, b"...")
        } else {
            said(b"Wait: ", &pending.name, b" is starting")
        };
        toast(ctx, &text, NotifyLevel::Info);
        return;
    }
    let before = children();
    match issue_load(name) {
        Issued::Loaded(pid) => {
            ctx.store_apps.installed_pids.insert(name.to_vec(), pid);
            boot(pid);
        }
        Issued::Refused(ERR_EXIST) => focus_running(ctx, name),
        Issued::Refused(status) => report_failure(ctx, name, status),
        Issued::InFlight => {
            ctx.store_apps.launch =
                Some(Launch::new(name, before, crate::server::dock_clock::now()));
            toast(ctx, &said(b"Starting ", name, b"..."), NotifyLevel::Info);
        }
    }
}
