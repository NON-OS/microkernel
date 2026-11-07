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

//! The shell turns the frame tracer on or off (frame_pacer/trace). Only the
//! shell: the lines name how busy the screen is.

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if !super::shell_only::from_shell(sender) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let Some(on) = crate::requests::toggles::frame_trace(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    if on != ctx.trace.is_on() {
        ctx.trace.set(on);
        crate::say::say(if on { "frame trace on" } else { "frame trace off" });
    }
    respond::status(sender, req, 0, tx)
}
