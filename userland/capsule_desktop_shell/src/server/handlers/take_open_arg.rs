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

//! An app's window asks whether the shell holds something for it to open. A
//! path is answered to any window of the app it was left for; a command line
//! (state::open_arg) only to a window of the Terminal, and only while still
//! wanted. The asker is known by the pid the kernel stamps on the call, so no
//! process can take another app's argument by claiming to be it.

use alloc::string::String;

use nonos_libc::mk_uptime_ms;

use crate::protocol::Request;
use crate::server::handlers::instances::app_of_pid;
use crate::server::respond;
use crate::state::apps::LAUNCHER_APPS;
use crate::state::open_arg::{still_wanted, TERMINAL};
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    if ctx.pending_command.as_ref().is_some_and(|c| !still_wanted(c.at_ms, mk_uptime_ms())) {
        ctx.pending_command = None;
    }
    // Nearly every ask finds nothing. Answering those before working out
    // which app is asking spares a registry lookup per instance of every app
    // on each tick of each window that asks.
    if ctx.pending_open.is_empty() && ctx.pending_command.is_none() {
        let _ = respond::status(sender_pid, req, 0, tx);
        return;
    }
    // The argument was left under the app's own name, but the window asking
    // may be any of its numbered instances.
    let svc = app_of_pid(sender_pid).and_then(|i| LAUNCHER_APPS.get(i)).map(|a| a.service);
    let arg = match svc {
        Some(s) if s == TERMINAL && ctx.pending_command.is_some() => {
            ctx.pending_command.take().map(|c| c.line)
        }
        _ => path_for(ctx, svc),
    };
    match arg {
        Some(a) => {
            let _ = respond::payload(sender_pid, req, a.as_bytes(), tx);
        }
        None => {
            let _ = respond::status(sender_pid, req, 0, tx);
        }
    }
}

fn path_for(ctx: &mut Context, svc: Option<&[u8]>) -> Option<String> {
    let name = core::str::from_utf8(svc?).ok()?;
    ctx.pending_open.remove(name)
}
