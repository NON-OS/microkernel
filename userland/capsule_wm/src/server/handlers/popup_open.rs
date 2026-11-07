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

use nonos_libc::mk_ipc_reply;

use crate::protocol::{response_header, write_status, Request, E_INVAL, E_NOENT, E_NOMEM, HDR_LEN};
use crate::server::respond;
use crate::state::Context;
use crate::window::popup::{decode, encode, open, POPUP_OPEN_RESP_LEN};
use crate::window::Visibility;

use super::window_open::menubar_for;

const STATUS_LEN: usize = 4;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some(ask) = decode(body) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    // Over a window of the caller's own that is on screen: no process opens a
    // popup over another's window.
    let Some(parent) = ctx
        .windows
        .find(sender_pid, ask.parent_id)
        .filter(|w| w.visibility == Visibility::Visible)
        .copied()
    else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    // A popup is opened once; an id the caller holds already is refused, as
    // window open refuses a table that has no room.
    if ctx.windows.find(sender_pid, ask.window_id).is_some() {
        let _ = respond::status(sender_pid, req, E_NOMEM, tx);
        return;
    }
    let z = ctx.z.allocate();
    let display = (ctx.display_width, ctx.display_height);
    // The shell's own popups are drawn in its chrome, over the menubar too.
    let top = match sender_pid == super::shell_pid::shell_pid() {
        true => 0,
        false => menubar_for(display.0, display.1),
    };
    let (window, placed) = open(&ask, &parent, z, display, top);
    if ctx.windows.insert(window).is_err() {
        let _ = respond::status(sender_pid, req, E_NOMEM, tx);
        return;
    }
    // No focus change and no notice to the shell: the popup is part of its
    // window. Its layer, submitted next, goes on top of its band, where this
    // stack has just put it.
    response_header(tx, req, (STATUS_LEN + POPUP_OPEN_RESP_LEN) as u32);
    write_status(tx, 0);
    let at = HDR_LEN + STATUS_LEN;
    tx[at..at + POPUP_OPEN_RESP_LEN].copy_from_slice(&encode(&placed));
    let _ = mk_ipc_reply(sender_pid, tx.as_ptr(), at + POPUP_OPEN_RESP_LEN);
}
