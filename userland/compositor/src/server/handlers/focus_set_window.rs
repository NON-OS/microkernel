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

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::state::{raise_rule, scene_raise, Context};

use super::focus_set::wm_pid;

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
    let Some((target_pid, window_id)) = crate::requests::place_keyed::focus_set_window(body) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // The window manager sends this for each window it raises, in its own
    // stacking order, so one window's layers come up and its owner's other
    // windows stay where they were: the window drawn on top at a point is the
    // window its hit test gives a click there to.
    ctx.focus.set(target_pid);
    if let Some(rect) = scene_raise::raise_window(&mut ctx.scene, target_pid, window_id) {
        ctx.damage.accumulate(rect);
    }
    respond::status(sender_pid, req, 0, tx)
}
