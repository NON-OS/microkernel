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

//! NCMP 0x000C SCREENSHOT: the scene composed into a surface the caller
//! registered and shared with the compositor, without the trusted band. Body: its handle u64.
//! Only from a holder of the screen capture bit (E_PERM otherwise). The
//! band shows a screen mark for three seconds, presented before the answer,
//! so a capture is never unseen; one log line names no process. Answer:
//! the width and height copied, u32 each.

use nonos_libc::{mk_cap_check, mk_surface_release, mk_uptime_ms};

use crate::display::screenshot_rule::{may_capture, NOTICE_MS};
use crate::frame_pacer::canvas_copy::capture;
use crate::frame_pacer::compose::Scene;
use crate::protocol::{Request, E_INVAL, E_IO, E_PERM};
use crate::server::respond;
use crate::state::attach_kernel::Kernel;
use crate::state::trusted_rule::TRUSTED_Z;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if !may_capture(sender, mk_cap_check) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let handle = crate::requests::capture::screenshot(body);
    let Some(handle) = handle else { return respond::status(sender, req, E_INVAL, tx) };
    let Some(dst) = super::attach_argb::attach_argb(handle) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    // The notice first: the capture is on the screen before it is taken.
    ctx.screen_notice_until = mk_uptime_ms() + NOTICE_MS;
    if crate::frame_pacer::show_band(ctx).is_err() {
        let _ = mk_surface_release(handle);
        return respond::status(sender, req, E_IO, tx);
    }
    let scene = Scene {
        scene: &mut ctx.scene,
        attach: &mut ctx.attach,
        kernel: &mut Kernel,
        damage: &mut ctx.damage,
        ceiling: TRUSTED_Z,
    };
    let (w, h) = capture(dst, (ctx.width, ctx.height), scene);
    let _ = mk_surface_release(handle);
    crate::say::say("[PRIVACY] screen captured");
    let mut out = [0u8; 8];
    (out[..4].copy_from_slice(&w.to_le_bytes()), out[4..].copy_from_slice(&h.to_le_bytes()));
    respond::status_payload(sender, req, 0, &out, tx)
}
