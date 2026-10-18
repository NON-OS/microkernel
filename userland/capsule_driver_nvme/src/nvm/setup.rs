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
use crate::error::NvmeResult;
use crate::regs::Regs;

pub fn bring_up(
    device_id: u64,
    epoch: u64,
    regs: Regs,
    stride: u8,
    admin: &mut AdminQueue,
    geometry: NamespaceGeometry,
) -> NvmeResult<IoQueue> {
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
