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

//! The transport: reset and features, the control queue's DMA and the
//! queue programmed, then bus mastering and DRIVER_OK. A device a dead
//! predecessor left live holds rings in frames since freed and runs them
//! once it may reach memory, so Bus Master goes on only after the reset was
//! read back and the queue moved to this grant, and never for a device
//! refused at any step before it. It used to go on before the queue's own
//! refusals. The cursor queue, where the device offers one, is moved to a
//! grant of its own in the same window; a grant that cannot be had leaves
//! the device without a cursor plane, and the compositor draws the pointer.

use nonos_virtio::common::driver_ok;
use nonos_virtio::{VirtioError, Window};

use super::dma;
use super::pci;
use crate::device::virtqueue::{CursorQueue, QueueLayout};
use crate::device::ControlQueue;
use crate::init::{cursor_offered, negotiate, program, program_cursor};
use crate::regs::Regs;

pub struct Ready {
    pub control_queue: ControlQueue,
    /// The cursor queue, when the device offers one and its grant was had.
    pub cursor_queue: Option<CursorQueue>,
    /// The features in force.
    pub features: u64,
}

pub fn start(device_id: u64, claim_epoch: u64, window: &Window) -> Result<Ready, &'static str> {
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message())?.region;
    let common = window.common.region;
    let features = negotiate(&common)?;
    let queue = dma::map_queue(device_id, claim_epoch)?;
    let started = program(&common, window.notify_area(), queue.device_addr)?;
    let layout = QueueLayout::new(started.queue_size, queue.view, queue.device_addr)?;
    let notify = window.notify.region;
    let regs = Regs { common, notify, doorbell: started.doorbell, device };
    let cursor = match cursor_offered(&common) {
        true => cursor_queue(device_id, claim_epoch, window, regs)?,
        false => None,
    };
    pci::command(device_id, claim_epoch, pci::MASTER)?;
    driver_ok(&common);
    let control_queue = ControlQueue::new(layout, regs);
    Ok(Ready { control_queue, cursor_queue: cursor, features })
}

/// The cursor queue on a grant of its own, or none if no grant was had.
fn cursor_queue(
    device_id: u64,
    claim_epoch: u64,
    window: &Window,
    regs: Regs,
) -> Result<Option<CursorQueue>, &'static str> {
    let Ok(ring) = dma::map_queue(device_id, claim_epoch) else { return Ok(None) };
    let (size, doorbell) = program_cursor(&regs.common, window.notify_area(), ring.device_addr)?;
    let layout = QueueLayout::new(size, ring.view, ring.device_addr)?;
    Ok(Some(CursorQueue::new(layout, Regs { doorbell, ..regs })))
}
