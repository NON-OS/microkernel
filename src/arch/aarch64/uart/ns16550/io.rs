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

use super::constants::{LSR, LSR_DR, LSR_THRE, RBR, THR};
use super::device::Ns16550;

impl Ns16550 {
    /// THRE: the holding register (or the mini-UART's FIFO) has room.
    pub(crate) fn tx_ready(&self) -> bool {
        self.regs.read(LSR) & LSR_THRE != 0
    }

    pub(crate) fn send(&self, byte: u8) {
        self.regs.write(THR, byte as u32);
    }

    pub(crate) fn receive(&self) -> Option<u8> {
        if self.regs.read(LSR) & LSR_DR == 0 {
            return None;
        }
        Some((self.regs.read(RBR) & 0xFF) as u8)
    }
}
