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

use nonos_libc::mk_service_lookup;

use crate::protocol::{Request, E_INVAL, E_PERM, FOCUS_SET_REQ_LEN};
use crate::server::respond;
use crate::state::{raise_rule, scene_raise, Context};

const WM_SERVICE: &[u8] = b"wm";

/// The window manager's pid now, looked up on every request so a restarted
/// window manager is recognised at once.
fn wm_pid() -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(WM_SERVICE.as_ptr(), WM_SERVICE.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if !raise_rule::may_raise(sender_pid, wm_pid()) {
        return respond::status(sender_pid, req, E_PERM, tx);
    }
    if body.len() != FOCUS_SET_REQ_LEN {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    let Some(target_pid) = super::u32_at(body, 0) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // The window manager sends this whenever a window is focused or raised in
    // its stack, so the target's layer goes on top of its band here too and
    // the two orders stay one order. Only the raised layer's rectangle can
    // show anything new, and it is repainted whole, so nothing is left
    // half-drawn under the window that came up.
    ctx.focus.set(target_pid);
    if let Some(rect) = scene_raise::raise_by_pid(&mut ctx.scene, target_pid) {
        ctx.damage.accumulate(rect);
    }
    respond::status(sender_pid, req, 0, tx)
}
