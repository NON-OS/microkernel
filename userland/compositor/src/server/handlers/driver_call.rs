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

//! One call to the display driver with F 12's bytes, for the brightness
//! and monitor-control ops: only to a driver that serves them, bounded so a
//! slow monitor's DDC/CI cannot hold the compositor's frames for long; and
//! the output a brightness key is for.

use nonos_libc::mk_ipc_call_timeout;

use crate::display::brightness_rule::{output_at, serves};
use crate::frame_pacer::lens::focus_centre;
use crate::state::Context;

/// A monitor's DDC/CI reply and its pacing fit well inside this.
const DRIVER_MS: u64 = 200;

/// The driver's port, when the driver driven now serves brightness.
pub fn brightness_port(ctx: &Context) -> Option<u32> {
    ctx.wire.filter(|w| serves(w.name)).map(|_| ctx.gfx_port)
}

/// The answer's length in `out`, its status at least; None when the
/// driver did not answer.
pub fn call(port: u32, req: &[u8], out: &mut [u8]) -> Option<usize> {
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        out.as_mut_ptr(),
        out.len(),
        DRIVER_MS,
    );
    (n >= 4).then(|| (n as usize).min(out.len()))
}

/// The status of a request whose answer has no body.
pub fn status(port: u32, req: &[u8]) -> Option<i32> {
    let mut out = [0u8; 8];
    call(port, req, &mut out).map(|_| i32::from_le_bytes([out[0], out[1], out[2], out[3]]))
}

/// The outputs shown now: the first, then each further head's.
pub fn outputs(ctx: &Context) -> ([u16; 4], usize) {
    let mut outputs = [0u16; 4];
    for (o, h) in outputs.iter_mut().skip(1).zip(&ctx.heads) {
        *o = h.output;
    }
    (outputs, (1 + ctx.heads.len()).min(4))
}

/// The output the focused window (else the pointer) is on.
pub fn focused_output(ctx: &Context) -> u16 {
    output_at(ctx.plan.as_ref(), &outputs(ctx).0, focus_centre(ctx).0)
}
