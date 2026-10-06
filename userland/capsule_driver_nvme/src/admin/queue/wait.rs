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

use super::constants::COMPLETION_TIMEOUT_MS;
use super::types::AdminQueue;
use crate::error::NvmeResult;
use crate::regs::Regs;

impl AdminQueue {
    /// Wait for admin command `cid`, which `what` names on the console should
    /// it fail, time out, or lose the clock it is timed on.
    pub(super) fn wait(&mut self, regs: Regs, stride: u8, cid: u16, what: &str) -> NvmeResult<()> {
        self.wait_ms(regs, stride, cid, what, COMPLETION_TIMEOUT_MS)
    }
}
