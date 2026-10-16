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

//! The first virtio-gpu transfer, scanout and flush a call completes,
//! marked once each for the boot bench.

use core::sync::atomic::AtomicBool;

static GPU_TRANSFER: AtomicBool = AtomicBool::new(false);
static GPU_SCANOUT: AtomicBool = AtomicBool::new(false);
static GPU_FLUSH: AtomicBool = AtomicBool::new(false);

/// `req` holds the request header, `resp + 20` the reply's status word.
pub(super) fn mark_first(name: &str, req: u64, req_len: usize, resp: u64, recv_result: i64) {
    if recv_result < 24 || req_len < 20 {
        return;
    }
    let mut hdr = [0u8; 20];
    let mut status = [0u8; 4];
    let ok_hdr = crate::usercopy::copy_from_user(req, &mut hdr).is_ok();
    let ok_status = crate::usercopy::copy_from_user(resp + 20, &mut status).is_ok();
    if name.as_bytes() == b"driver.virtio_gpu0" && ok_hdr && ok_status {
        let magic = u32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
        let op = u16::from_le_bytes([hdr[6], hdr[7]]);
        if magic == 0x4E56_4750 && u32::from_le_bytes(status) == 0 {
            if op == 0x0008 {
                crate::sys::bench::mark_once(&GPU_TRANSFER, b"virtio_gpu_transfer_first");
            }
            if op == 0x0009 {
                crate::sys::bench::mark_once(&GPU_SCANOUT, b"virtio_gpu_scanout_first");
            }
            if op == 0x000A {
                crate::sys::bench::mark_once(&GPU_FLUSH, b"virtio_gpu_flush_first");
            }
        }
    }
}
