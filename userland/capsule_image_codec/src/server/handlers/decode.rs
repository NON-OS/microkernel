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

use nonos_libc::{mk_surface_attach, mk_surface_release, mk_uptime_ms, SurfaceDescriptor};
use nonos_toolkit::image::types::ImageSize;

use crate::protocol::{DECODE_REQ_LEN, DECODE_RESP_LEN, E_BAD_LEN, HDR_LEN, Request, STATUS_LEN};
use crate::server::outputs::{Output, Outputs};
use crate::server::{handlers::surface::register_argb_surface, respond};

use super::decode_sized::decode_sized;

pub fn handle(outputs: &mut Outputs, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < DECODE_REQ_LEN {
        return fail(sender_pid, req, E_BAD_LEN, tx);
    }
    let in_handle = u64::from_le_bytes([body[0], body[1], body[2], body[3], body[4], body[5], body[6], body[7]]);
    let byte_len = u64::from_le_bytes([body[8], body[9], body[10], body[11], body[12], body[13], body[14], body[15]]) as usize;
    let mut desc = SurfaceDescriptor::default();
    let va = mk_surface_attach(in_handle, &mut desc as *mut _);
    if va <= 0 || (byte_len as u64) > desc.byte_len {
        if va > 0 { let _ = mk_surface_release(in_handle); }
        return fail(sender_pid, req, E_BAD_LEN, tx);
    }
    let img = unsafe { core::slice::from_raw_parts(va as usize as *const u8, byte_len) };
    let result = decode_sized(req.op, img);
    let _ = mk_surface_release(in_handle);
    match result {
        Ok((pixels, size)) => finish(outputs, sender_pid, req, &pixels, size, tx),
        Err(e) => fail(sender_pid, req, e, tx),
    }
}

fn fail(sender_pid: u32, req: &Request, errno: i32, tx: &mut [u8]) { let _ = respond::status(sender_pid, req, errno, tx); }

fn finish(outputs: &mut Outputs, sender_pid: u32, req: &Request, pixels: &[u32], size: ImageSize, tx: &mut [u8]) {
    let count = size.pixel_count() as usize;
    let (handle, stride, byte_len, (base, len)) = match register_argb_surface(&pixels[..count], size) {
        Ok(v) => v,
        Err(e) => return fail(sender_pid, req, e, tx),
    };
    let made = Output { client: sender_pid, base, len, made_ms: mk_uptime_ms() };
    if let Some(oldest) = outputs.hold(made) {
        crate::server::release_output(oldest);
    }
    let o = HDR_LEN + STATUS_LEN;
    tx[o..o + 8].copy_from_slice(&handle.to_le_bytes());
    tx[o + 8..o + 12].copy_from_slice(&size.width.to_le_bytes());
    tx[o + 12..o + 16].copy_from_slice(&size.height.to_le_bytes());
    tx[o + 16..o + 20].copy_from_slice(&stride.to_le_bytes());
    tx[o + 20..o + 24].copy_from_slice(&1u32.to_le_bytes());
    tx[o + 24..o + 32].copy_from_slice(&byte_len.to_le_bytes());
    let _ = respond::payload(sender_pid, req, DECODE_RESP_LEN, tx);
}

