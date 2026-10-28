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

use super::constants::{IO_ENTRIES, IO_QID};
use super::geometry::NamespaceGeometry;
use super::queue::IoQueue;
use crate::admin::AdminQueue;
use crate::controller::ControllerInfo;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

/// Build queue pair 1 for `geometry`. A controller whose CAP.MQES allows
/// fewer than IO_ENTRIES entries is refused before anything is mapped or
/// sent: Create I/O CQ and SQ would name a queue larger than it supports.
pub fn bring_up(
    device_id: u64,
    epoch: u64,
    regs: Regs,
    info: ControllerInfo,
    admin: &mut AdminQueue,
    geometry: NamespaceGeometry,
) -> NvmeResult<IoQueue> {
    if info.max_queue_entries() < IO_ENTRIES {
        return Err(NvmeError::QueueTooShallow);
    }
    let stride = info.doorbell_stride();
    let io = IoQueue::allocate(device_id, epoch, stride, IO_QID, &geometry)?;
    io.create(admin, regs, stride)?;
    Ok(io)
}

impl IoQueue {
    /// Create the queue pair on the controller, in this queue's memory: at
    /// bring-up, and again after a reset, which deletes every I/O queue.
    pub(crate) fn create(&self, admin: &mut AdminQueue, regs: Regs, stride: u8) -> NvmeResult<()> {
        admin.create_io_cq(regs, stride, IO_QID, IO_ENTRIES, self.cq.device_addr())?;
        admin.create_io_sq(regs, stride, IO_QID, IO_QID, IO_ENTRIES, self.sq.device_addr())
    }
}
