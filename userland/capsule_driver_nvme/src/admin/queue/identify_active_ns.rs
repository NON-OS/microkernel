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

use super::constants::IDENTIFY_SLICE_BYTES;
use super::types::AdminQueue;
use crate::admin::Submission;
use crate::error::NvmeResult;
use crate::regs::Regs;

impl AdminQueue {
    /// The active namespace list, from the first NSID on.
    pub fn identify_active_namespaces(&mut self, regs: Regs, stride: u8) -> NvmeResult<&[u8]> {
        let cid = self.cid;
        self.cid = self.cid.wrapping_add(1).max(1);
        self.submit(
            regs,
            stride,
            Submission::identify_active_ns_list(cid, 0, self.identify.device_addr()),
        );
        self.wait(regs, stride, cid, "identify active namespace list (CNS 02h)")?;
        Ok(unsafe {
            core::slice::from_raw_parts(self.identify.user_va() as *const u8, IDENTIFY_SLICE_BYTES)
        })
    }
}
