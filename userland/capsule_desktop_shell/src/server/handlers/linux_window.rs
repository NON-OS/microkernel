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

//! A Linux run names the package it started for, so the dock awaits its
//! window as the tile that package is, or as no app's
//! (state/taskbar/linux_run.rs): a minimised Qwen window is then raised by
//! its icon however Qwen was opened, and a store run's window is never
//! taken for Qwen's. Only the process registered as app.linux.run may name
//! one, by the pid the kernel stamps on the call, a name no other capsule
//! may take. The body is the package name alone.

use super::launcher_request::lookup_pid;
use super::run_linux::RUN;
use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::state::taskbar::LINUX_PACKAGE_MAX;
use crate::state::{run_named, Context};

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if sender_pid == 0 || lookup_pid(RUN) != Some(sender_pid) {
        let _ = respond::status(sender_pid, req, E_PERM, tx);
        return;
    }
    if body.is_empty() || body.len() > LINUX_PACKAGE_MAX {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let _ = run_named(&mut ctx.taskbar, body);
    let _ = respond::status(sender_pid, req, 0, tx);
}
