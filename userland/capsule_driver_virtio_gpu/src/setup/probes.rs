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

//! The best-effort probes after the scanouts: the panel's EDID, then the
//! 3D path. A host without EDID or a GL backend still gets the full 2D
//! scanout path, and the boot log records which one this is. A command
//! either probe sends may stall, which resets the device: the
//! queue is then restarted on the same grant, so the outputs still go up on
//! the 2D path. The reset queue used to be handed on, so every surface was
//! refused and the attempt failed: a host whose virgl stalled lost 2D with
//! it once the attempts ran out.

use nonos_gfx_serve::Panel;
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
    /// The monitor's EDID for output 0, or an empty panel.
    pub panel: Panel,
}

pub fn run(
    q: ControlQueue,
    features: u64,
    notify: NotifyArea,
    fences: &FenceCounter,
    (device_id, claim_epoch): (u64, u64),
) -> Result<Probed, &'static str> {
    let panel = edid::panel(&q, features & u64::from(VIRTIO_GPU_F_EDID) != 0);
    let (q, features) = in_service(q, features, notify)?;
    // Logged only: nothing on the display wire draws through the 3D path.
    let virgl = features & u64::from(VIRTIO_GPU_F_VIRGL) != 0;
    let _ = probe_3d::probe(&q, fences, virgl, device_id, claim_epoch);
    let (queue, _) = in_service(q, features, notify)?;
    Ok(Probed { queue, panel })
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
