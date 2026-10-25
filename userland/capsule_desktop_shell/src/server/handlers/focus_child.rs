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

//! OP_FOCUS_CHILD: a process that loaded an app asks the shell to give it the
//! focus frame it waits on, so its window opens; state::apps::focus_rule says
//! who may. The body is the app's pid, four bytes little-endian.

use super::launch_children::table;
use super::launch_wake::boot;
use crate::protocol::{Request, E_INVAL, E_NOENT, E_PERM};
use crate::server::respond;
use crate::state::apps::focus_rule::may_focus;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Ok(pid) = <[u8; 4]>::try_from(body).map(u32::from_le_bytes) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let status = match entry(pid) {
        None => E_NOENT,
        Some(parent) if may_focus(sender_pid, parent) => {
            boot(pid);
            0
        }
        Some(_) => E_PERM,
    };
    let _ = respond::status(sender_pid, req, status, tx);
}

/* The parent the process table shows for `pid`, if it is alive. */
fn entry(pid: u32) -> Option<u32> {
    table().into_iter().find(|e| pid != 0 && e.pid == pid).map(|e| e.ppid)
}
