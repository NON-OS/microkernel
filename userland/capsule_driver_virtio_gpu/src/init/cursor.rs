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

//! The cursor queue, when the device offers one of two entries or more:
//! laid out on its own grant as the control queue is, and programmed before
//! DRIVER_OK. A device that offers none is driven without a cursor plane.
//! One that offers the queue and then refuses it is FAILED, as a refused
//! control queue leaves it.

use nonos_virtio::common::{queue_max, setup_queue, NotifyArea, QueueSpec};
use nonos_virtio::{Mmio, VirtioError};

use crate::constants::{
    CURSORQ_INDEX, VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_MAX_SIZE, VQ_USED_OFFSET,
};

/// Whether the device offers a cursor queue the driver can use.
pub fn offered(common: &Mmio) -> bool {
    queue_max(common, CURSORQ_INDEX) >= 2
}

/// The cursor queue's size and doorbell.
pub fn program_cursor(
    common: &Mmio,
    notify: NotifyArea,
    queue_phys: u64,
) -> Result<(u16, usize), &'static str> {
    let offered = core::cmp::min(queue_max(common, CURSORQ_INDEX), VQ_MAX_SIZE);
    let size = offered.checked_ilog2().map_or(0, |log| 1 << log);
    let at = |off: usize| queue_phys.checked_add(off as u64).ok_or("virtio-gpu: ring address");
    let spec = QueueSpec {
        index: CURSORQ_INDEX,
        size,
        min_size: 2,
        desc: at(VQ_DESC_OFFSET)?,
        driver: at(VQ_AVAIL_OFFSET)?,
        device: at(VQ_USED_OFFSET)?,
        vector: None,
    };
    let ready = setup_queue(common, &spec, notify).map_err(VirtioError::message)?;
    Ok((ready.size, ready.notify_offset))
}
