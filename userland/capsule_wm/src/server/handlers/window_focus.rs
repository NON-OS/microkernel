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

use crate::protocol::{Request, E_INVAL, E_NOENT, E_PERM, WINDOW_FOCUS_REQ_LEN};
use crate::server::respond;
use crate::server::tell_compositor::lift;
use crate::state::Context;
use crate::window::Visibility;

/// A client focusing its own window. Focus is this table's: it is set here
/// whatever the compositor says. It was set only once the compositor had
/// answered the focus_set within its 16 ms, and a compositor composing a
/// whole frame often had not, so a window brought back from the dock was
/// drawn on top while the keys went on to the window it covered. A window
/// not on screen (minimised) takes no focus; it is restored first.
pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != WINDOW_FOCUS_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(window_id) = super::u32_at::u32_at(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(window) = ctx.windows.find(sender_pid, window_id) else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    if !window.kind.focusable() || window.visibility != Visibility::Visible {
        let _ = respond::status(sender_pid, req, E_PERM, tx);
        return;
    }
    if ctx.focus.set(sender_pid, window_id) {
        lift(ctx, sender_pid);
    }
    let _ = respond::status(sender_pid, req, 0, tx);
}
