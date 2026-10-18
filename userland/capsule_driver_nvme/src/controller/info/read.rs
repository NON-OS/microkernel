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

use super::info_type::ControllerInfo;
use crate::constants::{
    REG_AQA, REG_CAP, REG_CC, REG_CMBLOC, REG_CMBSZ, REG_CSTS, REG_INTMC, REG_INTMS, REG_VS,
};
use crate::regs::Regs;

impl ControllerInfo {
    /// The register block's first 0x40 bytes, from a BAR0 mapping the broker
    /// gave. Read before anything says the block is NVMe's.
    pub fn read(regs: Regs) -> Self {
        // SAFETY: every register read lies in the first 0x40 bytes of BAR0.
        // The broker refuses a window whose first page holds an MSI-X table,
        // so that whole page is mapped: BAR0 is at least 16 KiB
        // (discover::pci_match) and, like every memory BAR, size aligned.
        unsafe {
            Self {
                cap: regs.r64(REG_CAP),
                version: regs.r32(REG_VS),
                cc: regs.r32(REG_CC),
                csts: regs.r32(REG_CSTS),
                aqa: regs.r32(REG_AQA),
                intms: regs.r32(REG_INTMS),
                intmc: regs.r32(REG_INTMC),
                cmbloc: regs.r32(REG_CMBLOC),
                cmbsz: regs.r32(REG_CMBSZ),
            }
        }
    }
}
