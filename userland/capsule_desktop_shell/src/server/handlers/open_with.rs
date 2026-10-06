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

use alloc::string::String;

use super::launcher_request::LaunchOutcome;
use crate::protocol::{Request, E_BUSY, E_INVAL, E_NOENT};
use crate::server::respond;
use crate::state::apps::LAUNCHER_APPS;
use crate::state::open_arg::is_path;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 2 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let nlen = u16::from_le_bytes([body[0], body[1]]) as usize;
    if body.len() < 2 + nlen {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let svc = &body[2..2 + nlen];
    let path = &body[2 + nlen..];
    let Some(index) = LAUNCHER_APPS.iter().position(|a| a.service == svc) else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    // Only a path may be left this way. Any process can send OP_OPEN_WITH, and
    // what it leaves is what the app is told to open, so it must never take
    // the form of the Terminal's command line (state::open_arg).
    let path = match core::str::from_utf8(path) {
        Ok(p) if is_path(p) => p,
        _ => {
            let _ = respond::status(sender_pid, req, E_INVAL, tx);
            return;
        }
    };
    // The file manager shows a preview and says the app could not be
    // started when the answer is not 0.
    let status = match super::hand_over::hand_over(ctx, index, String::from(path)) {
        LaunchOutcome::Failed => E_BUSY,
        _ => 0,
    };
    let _ = respond::status(sender_pid, req, status, tx);
}
