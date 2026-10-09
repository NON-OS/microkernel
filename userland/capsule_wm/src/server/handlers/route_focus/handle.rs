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

use crate::focus::press::Refused;
use crate::focus::press_focus;
use crate::protocol::{Request, E_INVAL, E_NOENT, E_PERM, ROUTE_FOCUS_REQ_LEN};
use crate::server::respond;
use crate::state::Context;

use super::is_input_router::is_input_router;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != ROUTE_FOCUS_REQ_LEN || !is_input_router(ctx, sender_pid) {
        if respond::status(sender_pid, req, E_PERM, tx) < 0 {
            return;
        }
        return;
    }
    let Some(owner_pid) = super::super::u32_at::u32_at(body, 0) else {
        if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
            return;
        }
        return;
    };
    let Some(window_id) = super::super::u32_at::u32_at(body, 4) else {
        if respond::status(sender_pid, req, E_INVAL, tx) < 0 {
            return;
        }
        return;
    };
    // Focus and raise in one step, before the router delivers the press, so the
    // press that lands on a window's title bar both brings the window up and
    // starts its drag, and the next hit test already sees it on top.
    let pressed =
        match press_focus(&mut ctx.windows, &mut ctx.z, &mut ctx.focus, owner_pid, window_id) {
            Ok(p) => p,
            Err(why) => {
                let errno = match why {
                    Refused::NoWindow => E_NOENT,
                    Refused::NotFocusable => E_PERM,
                };
                if respond::status(sender_pid, req, errno, tx) < 0 {
                    return;
                }
                return;
            }
        };
    if pressed.restacked() {
        crate::server::tell_compositor::lift(ctx, owner_pid);
    }
    if respond::status(sender_pid, req, 0, tx) < 0 {
        return;
    }
}
