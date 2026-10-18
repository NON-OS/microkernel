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

//! The best-effort probes after the scanouts: the panel's size, then the 3D
//! path. A host without EDID or a GL backend still gets the full 2D scanout
//! path, and the boot log records which one this is. A command either probe
//! sends may stall, which resets the device (plan/50 C1): the queue is then
//! restarted on the same grant, so the primary surface still goes up on the
//! 2D path, with virgl off. The reset queue used to be handed on, so the
//! surface was refused and the attempt failed: a host whose virgl stalled
//! lost 2D with it once the attempts ran out.

use nonos_libc::mk_debug;
use nonos_virtio::common::NotifyArea;

use super::{edid, probe_3d};
use crate::constants::{VIRTIO_GPU_F_EDID, VIRTIO_GPU_F_VIRGL};
use crate::device::ControlQueue;
use crate::init::restart;
use crate::state::FenceCounter;

const RESTARTED: &[u8] = b"[GPU] probe stalled; device reset, control queue restarted for 2d";

pub struct Probed {
    pub queue: ControlQueue,
    /// The features in force: a restart settles them again.
    pub features: u64,
    /// Render context 1 exists on the host: RESOURCE_CREATE_3D and
    /// SUBMIT_3D are usable.
    pub virgl_ready: bool,
}

pub fn run(
    q: ControlQueue,
    features: u64,
    notify: NotifyArea,
    fences: &FenceCounter,
    device_id: u64,
    claim_epoch: u64,
) -> Result<Probed, &'static str> {
    edid::fetch(&q, features & u64::from(VIRTIO_GPU_F_EDID) != 0);
    let (q, features) = in_service(q, features, notify)?;
    let virgl = features & u64::from(VIRTIO_GPU_F_VIRGL) != 0;
    // A stall reset the device, and the render context with it.
    let virgl_ready = probe_3d::probe(&q, fences, virgl, device_id, claim_epoch) && !q.is_reset();
    let (queue, features) = in_service(q, features, notify)?;
    Ok(Probed { queue, features, virgl_ready })
}

/// The queue as it is, or restarted if a probe's command stalled on it.
fn in_service(
    q: ControlQueue,
    features: u64,
    notify: NotifyArea,
) -> Result<(ControlQueue, u64), &'static str> {
    if !q.is_reset() {
        return Ok((q, features));
    }
    mk_debug(RESTARTED.as_ptr(), RESTARTED.len());
    restart(&q, notify)
}
