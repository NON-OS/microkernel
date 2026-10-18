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
use super::state::CommandRing;
use crate::regs::mmio_read32;

const CRCR_CRR: u32 = 1 << 3;
impl CommandRing {
    /// The controller the ring was programmed into, whose CRCR an abort
    /// writes.
    pub(crate) fn set_op_base(&mut self, op_base: u64) {
        self.op_base = op_base;
    }

    /// Whether an abort found the controller wedged: CRR never dropped.
    pub fn wedged(&self) -> bool {
        self.wedged
    }
}

/// CRR: the ring at `crcr` is running.
pub(super) fn running(crcr: u64) -> bool {
    // SAFETY: `crcr` is CRCR among the operational registers
    // `program_command_ring` gave the ring, inside the mapped window.
    let now = unsafe { mmio_read32(crcr) };
    now & CRCR_CRR != 0
}
