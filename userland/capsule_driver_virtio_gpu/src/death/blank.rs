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
//! The screen a dead or silent owner leaves, in an order that never
//! shows its last frame. Every scanout showing a resource is detached first
//! (SET_SCANOUT naming resource 0, as recalled from the virtio-gpu 2D
//! commands; unverified). Then the owner's own resources are released, the
//! primary is zeroed and moved to the host whole, so the host's copy holds
//! no frame either, and last the driver's blank is shown on scanout 0. A
//! step the device refuses or drops does not stop the rest; the answer says
//! whether all of them landed, so the watch tries again on its next wake.

use super::release::release;
use crate::constants::VG_MAX_SCANOUTS;
use crate::device::cmd::{self, transfer_to_host_2d::Rect};
use crate::driver::Driver;
use crate::state::Scanout;

pub fn blank(d: &Driver) -> bool {
    let mut whole = true;
    for id in 0..VG_MAX_SCANOUTS as u32 {
        whole &= detach(d, id);
    }
    whole &= release(d, d.owner.pid.get());
    whole &= zero_primary(d);
    whole & show(d)
}

fn detach(d: &Driver, id: u32) -> bool {
    let Some(s) = d.scanouts.get(id).filter(|s| s.current_resource_id != 0) else { return true };
    let rect = Rect { x: s.x, y: s.y, width: s.width.max(1), height: s.height.max(1) };
    let off = cmd::set_scanout(&d.control_queue, d.fences.issue(), id, 0, rect).is_ok();
    if off {
        d.scanouts.record(id, Scanout { current_resource_id: 0, ..s });
    }
    off
}

/// The primary zeroed where this driver maps it, then sent to the host.
pub fn zero_primary(d: &Driver) -> bool {
    let Some(p) = d.primary else { return true };
    // SAFETY: `user_va` maps the primary's `backing_len` bytes for as long
    // as this driver runs.
    unsafe { core::ptr::write_bytes(p.user_va as *mut u8, 0, p.backing_len as usize) };
    let rect = Rect { x: 0, y: 0, width: p.width, height: p.height };
    cmd::transfer_to_host_2d(&d.control_queue, d.fences.issue(), p.resource_id, rect, 0).is_ok()
}

/// The blank on scanout 0; with none, the scanouts stay detached and dark.
fn show(d: &Driver) -> bool {
    let Some(b) = d.blank else { return true };
    let rect = Rect { x: 0, y: 0, width: b.width, height: b.height };
    let q = &d.control_queue;
    if cmd::set_scanout(q, d.fences.issue(), 0, b.resource_id, rect).is_err() {
        return false;
    }
    let s = d.scanouts.get(0).unwrap_or_default();
    let (width, height) = (b.width, b.height);
    let on = Scanout { width, height, current_resource_id: b.resource_id, enabled: true, ..s };
    d.scanouts.record(0, on);
    cmd::resource_flush(q, d.fences.issue(), b.resource_id, rect).is_ok()
}
