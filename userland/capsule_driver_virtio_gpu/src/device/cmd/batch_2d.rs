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

//! A rectangle list as batches: every TRANSFER_TO_HOST_2D queued, one
//! notify, one wait; then every RESOURCE_FLUSH the same way. The device
//! may run a queue's commands in any order, so the flushes go out only
//! once every transfer they show has been answered. Two exits to the host
//! for the list, where one command at a time cost two per rectangle.

use super::flush::{self, REQ_LEN as FLUSH_LEN};
use super::hdr::{Hdr, RESP_HDR_LEN};
use super::transfer_to_host_2d::{self as transfer, Rect, REQ_LEN as TRANSFER_LEN};
use crate::constants::VG_RESP_OK_NODATA;
use crate::device::virtqueue::batch::submit_all;
use crate::device::virtqueue::ControlQueue;
use crate::state::FenceCounter;

fn answered_ok(a: &[u8; RESP_HDR_LEN]) -> bool {
    Hdr::parse(a).is_some_and(|h| h.type_ == VG_RESP_OK_NODATA)
}

fn drawn(r: &Rect) -> bool {
    r.width > 0 && r.height > 0
}

/// Each of `rects` of resource `id` (`width` pixels a row, four bytes a
/// pixel) copied from its backing to the host's copy.
pub fn transfer_rects(
    q: &ControlQueue,
    fences: &FenceCounter,
    (id, width): (u32, u32),
    rects: &mut dyn Iterator<Item = Rect>,
) -> Result<(), &'static str> {
    let row = width as u64 * 4;
    let mut reqs = rects.filter(drawn).filter_map(|r| {
        let at = r.y as u64 * row + r.x as u64 * 4;
        transfer::encode(fences.issue(), id, r, at).ok()
    });
    if id == 0 {
        return Err("virtio-gpu: invalid transfer args");
    }
    submit_all::<TRANSFER_LEN, RESP_HDR_LEN>(q, &mut reqs, &answered_ok)
}

/// Each of `rects` of resource `id` flushed to the scanouts showing it.
pub fn flush_rects(
    q: &ControlQueue,
    fences: &FenceCounter,
    id: u32,
    rects: &mut dyn Iterator<Item = Rect>,
) -> Result<(), &'static str> {
    let mut reqs = rects.filter(drawn).filter_map(|r| flush::encode(fences.issue(), id, r).ok());
    if id == 0 {
        return Err("virtio-gpu: invalid flush args");
    }
    submit_all::<FLUSH_LEN, RESP_HDR_LEN>(q, &mut reqs, &answered_ok)
}
