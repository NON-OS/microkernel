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

//! Boot bench marks for the first frames a display driver shows: the first
//! answered call of each kind to any driver in `displays.rs` whose request
//! is an NVGP frame (magic 0x4E56_4750) and whose reply says 0. The legacy
//! virtio-gpu ops keep their names; FLUSH (0x14) is the wire's own.

use crate::services::registry::displays::DISPLAYS;
use core::sync::atomic::AtomicBool;

static GPU_TRANSFER: AtomicBool = AtomicBool::new(false);
static GPU_SCANOUT: AtomicBool = AtomicBool::new(false);
static GPU_FLUSH: AtomicBool = AtomicBool::new(false);
static DISPLAY_FLUSH: AtomicBool = AtomicBool::new(false);

const NVGP: u32 = 0x4E56_4750;

/// After a call to `name` of `req_len` bytes at `req` that got
/// `recv_result` bytes back at `resp`.
pub(super) fn display_marks(name: &str, req: u64, req_len: usize, resp: u64, recv_result: i64) {
    if recv_result < 24 || req_len < 20 || !DISPLAYS.contains(&name) {
        return;
    }
    let (mut hdr, mut status) = ([0u8; 20], [0u8; 4]);
    let ok_hdr = crate::usercopy::copy_from_user(req, &mut hdr).is_ok();
    let ok_status = crate::usercopy::copy_from_user(resp + 20, &mut status).is_ok();
    let magic = u32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
    if !ok_hdr || !ok_status || magic != NVGP || u32::from_le_bytes(status) != 0 {
        return;
    }
    let (flag, event): (&AtomicBool, &[u8]) = match u16::from_le_bytes([hdr[6], hdr[7]]) {
        0x0008 => (&GPU_TRANSFER, b"virtio_gpu_transfer_first"),
        0x0009 => (&GPU_SCANOUT, b"virtio_gpu_scanout_first"),
        0x000A => (&GPU_FLUSH, b"virtio_gpu_flush_first"),
        0x0014 => (&DISPLAY_FLUSH, b"display_flush_first"),
        _ => return,
    };
    crate::sys::bench::mark_once(flag, event);
}
