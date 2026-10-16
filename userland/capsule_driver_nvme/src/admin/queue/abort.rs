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

use super::types::AdminQueue;
use crate::admin::Submission;
use crate::error::NvmeResult;
use crate::regs::Regs;

impl AdminQueue {
    /// Ask the controller to abort command `target` on I/O submission queue
    /// `sqid`, whose own wait ran out, and say whether it did. The Abort has
    /// the admin deadline of its own; no answer to it means the admin queue
    /// is stuck too. Answered, DW0 bit 0 of its completion is set when the
    /// command was not aborted (NVMe 1.4, 5.1); the rest of DW0 is reserved.
    pub fn abort(&mut self, regs: Regs, stride: u8, sqid: u16, target: u16) -> NvmeResult<bool> {
        let cid = self.cid;
        self.cid = self.cid.wrapping_add(1).max(1);
        self.submit(regs, stride, Submission::abort(cid, sqid, target));
        self.wait(regs, stride, cid, "abort an I/O command given up on")?;
        Ok(self.last().dw0 & 1 == 0)
    }
}
