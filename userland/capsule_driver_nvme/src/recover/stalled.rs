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

use super::plan::Stuck;
use super::rebuild::rebuild;
use crate::admin::AdminQueue;
use crate::error::NvmeResult;
use crate::nvm::{IoQueue, IO_QID};
use crate::setup::Device;

/// The controller, its queues and its host memory buffer, as a request is
/// served on them: the I/O queue is gone once it will not serve again.
pub struct Serving<'a> {
    pub device: &'a mut Device,
    pub admin: &'a mut AdminQueue,
    pub io: &'a mut Option<IoQueue>,
    pub hmb: Option<[u32; 5]>,
}

/// The controller, its queues and its host memory buffer, while an I/O
/// command it was given is stuck.
pub struct Stalled<'a> {
    pub device: &'a mut Device,
    pub admin: &'a mut AdminQueue,
    pub io: &'a mut IoQueue,
    /// The SET FEATURES dwords of the host memory buffer, if it has one.
    pub hmb: Option<[u32; 5]>,
}

impl Stuck for Stalled<'_> {
    fn csts(&mut self) -> u32 {
        self.device.csts()
    }

    fn abort(&mut self) -> NvmeResult<bool> {
        // Nothing out: nothing for the controller to give back.
        let Some(cid) = self.io.stuck() else { return Ok(true) };
        let stride = self.device.info.doorbell_stride();
        self.admin.abort(self.device.regs, stride, IO_QID, cid)
    }

    fn grace(&mut self, ms: u64) -> NvmeResult<()> {
        self.io.settle_within(self.device.regs, ms)
    }

    fn reset(&mut self) -> NvmeResult<()> {
        rebuild(self.device, self.admin, self.io, self.hmb)
    }
}
