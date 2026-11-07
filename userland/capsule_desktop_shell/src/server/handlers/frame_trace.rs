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

//! Ctrl+Alt+Shift+F: the compositor's per-stage frame trace on or off. The
//! lines go to the debug channel; the toast says which way it went.

use crate::compositor_client::frame_trace::frame_trace;
use crate::state::{Context, NotifyLevel};

pub fn toggle(ctx: &mut Context) {
    let on = !ctx.readings.frame_trace;
    let rid = ctx.issue_request_id();
    let now = crate::server::toast_clock::now();
    match frame_trace(ctx.ports.compositor_port, rid, on) {
        Ok(0) => {
            ctx.readings.frame_trace = on;
            let said: &[u8] = if on { b"Frame trace on" } else { b"Frame trace off" };
            ctx.toasts.push(said, NotifyLevel::Info, now);
        }
        _ => {
            ctx.toasts.push(b"The compositor did not take the frame trace", NotifyLevel::Error, now)
        }
    }
}
