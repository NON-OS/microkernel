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
    /// SET FEATURES `fid` with dwords 11 to 15, named `what` on failure.
    pub fn set_features(
        &mut self,
        regs: Regs,
        stride: u8,
        fid: u8,
        dw: [u32; 5],
        what: &str,
        timeout_ms: u64,
    ) -> NvmeResult<()> {
        let cid = self.cid;
        self.cid = self.cid.wrapping_add(1).max(1);
        self.submit(regs, stride, Submission::set_features(cid, fid, dw));
        self.wait_ms(regs, stride, cid, what, timeout_ms)
    }
}
