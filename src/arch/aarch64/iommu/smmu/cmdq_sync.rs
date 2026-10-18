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

use crate::memory::iommu::IommuError;

use super::cmdq::Cmdq;
use super::encode::cmd::{self, Command};
use super::encode::queue::consumed;
use super::mmio::Regs;
use super::regs::CMDQ_CONS;
use super::wait;

impl Cmdq {
    /// Queue a sync and wait for the unit to reach it, so every command
    /// before it has taken effect. A command the unit refused stops the
    /// queue with an error code in CONS; that, or a unit that never gets
    /// there, fails the call.
    pub(super) fn sync(&mut self, regs: Regs) -> Result<(), IommuError> {
        self.push(regs, cmd::sync())?;
        let (target, log2) = (self.prod, self.log2);
        let reached = wait::until(|| {
            let cons = regs.read32(CMDQ_CONS);
            cmd::cons_error(cons) != 0 || consumed(cons, target, log2)
        });
        let cons = regs.read32(CMDQ_CONS);
        if !reached || cmd::cons_error(cons) != 0 {
            return Err(IommuError::BackendFault);
        }
        Ok(())
    }

    /// Queue every command in `commands`, then sync.
    pub(super) fn run(&mut self, regs: Regs, commands: &[Command]) -> Result<(), IommuError> {
        for &command in commands {
            self.push(regs, command)?;
        }
        self.sync(regs)
    }
}
