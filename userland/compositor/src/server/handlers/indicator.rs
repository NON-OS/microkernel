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

//! NCMP 0x000A INDICATOR (Decision 9): a capturing driver says its camera
//! or microphone is on or off. Body: kind u32 (1 camera, 2 microphone), on
//! u32 (1 on, 0 off). Taken only from a process holding that kind's capture
//! bit (E_PERM otherwise). The band is drawn and presented before the
//! answer, so a driver that waits for 0 cannot start capturing unseen; a
//! present that fails takes the change back and answers E_IO. One log line
//! per change of what is shown, naming no device and no process.

use nonos_libc::mk_cap_check;

use crate::display::indicator_rule::may_indicate;
use crate::protocol::{Request, E_BUSY, E_INVAL, E_IO, E_PERM};
use crate::server::respond;
use crate::state::indicator::{Refused, KIND_CAMERA};
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some((kind, on)) = crate::requests::toggles::indicator(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    if !may_indicate(sender, kind, mk_cap_check) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let before = ctx.indicators;
    let status = match ctx.indicators.apply(kind, on, sender) {
        Err(Refused::Kind) => E_INVAL,
        Err(Refused::Full) => E_BUSY,
        Ok(false) => 0,
        Ok(true) if crate::frame_pacer::show_band(ctx).is_err() => {
            ctx.indicators = before;
            E_IO
        }
        Ok(true) => {
            let what = if kind == KIND_CAMERA { "camera" } else { "microphone" };
            let state = if on { "on" } else { "off" };
            crate::say::say(&alloc::format!("[PRIVACY] {} indicator {}", what, state));
            0
        }
    };
    respond::status(sender, req, status, tx)
}
