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

//! The transport: reset and features, the control queue's DMA, bus
//! mastering, the control queue and DRIVER_OK.

use nonos_virtio::{VirtioError, Window};

use super::dma::{self, QueueGrant};
use super::pci;
use crate::device::virtqueue::QueueLayout;
use crate::device::ControlQueue;
use crate::init::{negotiate, program};
use crate::regs::Regs;

pub struct Ready {
    pub control_queue: ControlQueue,
    /// The features in force.
    pub features: u64,
    pub queue: QueueGrant,
}

pub fn start(device_id: u64, claim_epoch: u64, window: &Window) -> Result<Ready, &'static str> {
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message())?.region;
    let common = window.common.region;
    let features = negotiate(&common)?;
    let queue = dma::map_queue(device_id, claim_epoch)?;
    pci::command(device_id, claim_epoch, pci::MASTER)?;
    let started = program(&common, window.notify_area(), queue.device_addr)?;
    let notify = window.notify.region;
    let regs = Regs { common, notify, doorbell: started.doorbell, device };
    let layout = QueueLayout::new(started.queue_size, queue.view, queue.device_addr)?;
    Ok(Ready { control_queue: ControlQueue::new(layout, regs), features, queue })
}
