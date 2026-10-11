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

//! The command queue: one frame of 256 commands, written by the kernel and
//! consumed by the unit.

use crate::memory::iommu::IommuError;

use super::encode::cmd::Command;
use super::encode::queue::{advance, index, is_full};
use super::mmio::Regs;
use super::regs::{BASE_RA, CMDQ_CONS, CMDQ_PROD};
use super::{frame, wait};

pub(super) struct Cmdq {
    pub(super) phys: u64,
    pub(super) log2: u8,
    pub(super) prod: u32,
}

impl Cmdq {
    /// A queue of one frame, or smaller if the unit takes fewer commands.
    pub(super) fn new(max_log2: u8) -> Result<Self, IommuError> {
        Ok(Self { phys: frame::alloc()?, log2: max_log2.min(8), prod: 0 })
    }

    pub(super) fn base(&self) -> u64 {
        BASE_RA | self.phys | u64::from(self.log2)
    }

    /// Queue `command`, waiting a bounded time for room.
    pub(super) fn push(&mut self, regs: Regs, command: Command) -> Result<(), IommuError> {
        let log2 = self.log2;
        if !wait::until(|| !is_full(self.prod, regs.read32(CMDQ_CONS), log2)) {
            return Err(IommuError::BackendFault);
        }
        let slot = index(self.prod, log2) as usize * 2;
        let words = frame::words(self.phys)?;
        words[slot] = command[0];
        words[slot + 1] = command[1];
        wait::publish();
        self.prod = advance(self.prod, log2);
        regs.write32(CMDQ_PROD, self.prod);
        Ok(())
    }
}
