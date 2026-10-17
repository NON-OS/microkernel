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
//! The driver's blank surface, made once at bring-up: DMA the size
//! of the primary (zeroed by the kernel when mapped, and mapped by no
//! other process), a resource on it, and one whole transfer, so the host's
//! copy is zero whatever the host put in it when made. One that cannot be
//! made is given back and left out: the watch then still detaches a dead
//! owner's scanouts, and the screen is left dark instead.

use super::primary_surface::{dma, Primary};
use crate::constants::VG_FORMAT_B8G8R8A8_UNORM;
use crate::device::cmd::{self, transfer_to_host_2d::Rect};
use crate::device::virtqueue::ControlQueue;
use crate::state::{Blank, FenceCounter, Resource, ResourceTable};

pub fn create(
    (device_id, claim_epoch): (u64, u64),
    q: &ControlQueue,
    fences: &FenceCounter,
    resources: &ResourceTable,
    p: &Primary,
) -> Option<Blank> {
    let len = p.stride as u64 * p.height as u64;
    let map = dma::map(device_id, claim_epoch, len).ok()?;
    let (id, width, height) = (resources.alloc_id(), p.width, p.height);
    let rect = Rect { x: 0, y: 0, width, height };
    let format = VG_FORMAT_B8G8R8A8_UNORM;
    let made = cmd::create_resource_2d(q, fences.issue(), id, format, width, height).is_ok();
    let attached =
        made && cmd::attach_backing(q, fences.issue(), id, map.device_addr, len as u32).is_ok();
    let sent = attached && cmd::transfer_to_host_2d(q, fences.issue(), id, rect, 0).is_ok();
    let entry = Resource {
        resource_id: id,
        owner_pid: 0,
        width,
        height,
        format,
        backing_addr: map.device_addr,
        backing_len: len as u32,
        in_use: true,
    };
    if sent && resources.insert(entry).is_ok() {
        return Some(Blank { resource_id: id, width, height });
    }
    // Given back in order: the host lets go of the pages before they go.
    if attached {
        let _ = cmd::detach_backing(q, fences.issue(), id);
    }
    if made {
        let _ = cmd::resource_unref(q, fences.issue(), id);
    }
    let _ = dma::rollback(map.grant_id, "virtio-gpu: blank dma rollback failed");
    None
}
