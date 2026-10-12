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

//! NCMP 0x000C SCREENSHOT: the composed canvas copied into a surface the
//! caller registered and shared with the compositor. Body: its handle u64.
//! Only from a holder of the screen capture bit (E_PERM otherwise). The
//! band shows a screen mark for three seconds, presented before the answer,
//! so a capture is never unseen; one log line names no process. Answer:
//! the width and height copied, u32 each.

use nonos_libc::{mk_cap_check, mk_surface_attach, mk_surface_release, mk_uptime_ms};
use nonos_libc::{SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

use crate::display::screenshot_rule::{may_capture, NOTICE_MS};
use crate::frame_pacer::canvas_copy::copy_canvas;
use crate::protocol::{Request, E_INVAL, E_IO, E_PERM, SCREENSHOT_REQ_LEN};
use crate::server::respond;
use crate::state::Context;
use crate::sw_blitter::Surface;

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
    let handle = super::u64_at(body, 0).filter(|_| body.len() == SCREENSHOT_REQ_LEN);
    let Some(handle) = handle else { return respond::status(sender, req, E_INVAL, tx) };
    let mut d = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut d);
    if va <= 0 || d.format != SURFACE_FORMAT_ARGB8888 {
        let _ = (va > 0).then(|| mk_surface_release(handle));
        return respond::status(sender, req, E_INVAL, tx);
    }
    // The notice first: the capture is on the screen before it is taken.
    ctx.screen_notice_until = mk_uptime_ms() + NOTICE_MS;
    if crate::frame_pacer::show_band(ctx).is_err() {
        let _ = mk_surface_release(handle);
        return respond::status(sender, req, E_IO, tx);
    }
    let (base_va, stride, width, height) = (va as u64, d.stride, d.width, d.height);
    let dst = Surface { base_va, stride, width, height, byte_len: d.byte_len };
    let src = Surface {
        base_va: ctx.backing_va,
        stride: ctx.stride,
        width: ctx.width,
        height: ctx.height,
        byte_len: ctx.backing_len,
    };
    let (w, h) = copy_canvas(src, dst);
    let _ = mk_surface_release(handle);
    crate::say::say("[PRIVACY] screen captured");
    let mut out = [0u8; 8];
    (out[..4].copy_from_slice(&w.to_le_bytes()), out[4..].copy_from_slice(&h.to_le_bytes()));
    respond::status_payload(sender, req, 0, &out, tx)
}
