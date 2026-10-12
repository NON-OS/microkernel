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

//! The control queue brought back after a reset, on the same grant
//!. The device is reset and its features settled again, the
//! ring region wiped, and the queue programmed and DRIVER_OK set. A reset
//! device counts from zero, so a stale index left in the rings would read
//! as commands published, or answered, that never were; the driver's own
//! count starts over with the new queue.

use nonos_virtio::common::NotifyArea;

use super::{negotiate, program};
use crate::constants::VQ_REGION_SIZE;
use crate::device::virtqueue::QueueLayout;
use crate::device::ControlQueue;
use crate::regs::Regs;

/// The queue to use from now on, and the features in force.
pub fn restart(q: &ControlQueue, notify: NotifyArea) -> Result<(ControlQueue, u64), &'static str> {
    let (common, ring, at) = (q.regs.common, q.layout.ring, q.layout.region_device_addr);
    let features = negotiate(&common)?;
    // The device is reset (read back by the clock): nothing reads the rings.
    for off in (0..VQ_REGION_SIZE).step_by(4) {
        ring.w32(off, 0);
    }
    let started = program(&common, notify, at)?;
    let layout = QueueLayout::new(started.queue_size, ring, at)?;
    let regs = Regs { doorbell: started.doorbell, ..q.regs };
    Ok((ControlQueue::new(layout, regs), features))
}
