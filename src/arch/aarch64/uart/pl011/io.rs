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

use super::constants::{FR_RXFE, FR_TXFF, INTERRUPT_CLEAR_ALL, UARTDR, UARTFR, UARTICR};
use super::device::Pl011;

impl Pl011 {
    pub(crate) fn tx_ready(&self) -> bool {
        self.read_reg(UARTFR) & FR_TXFF == 0
    }

    pub(crate) fn send(&self, byte: u8) {
        self.write_reg(UARTDR, byte as u32);
    }

    pub(crate) fn receive(&self) -> Option<u8> {
        if self.read_reg(UARTFR) & FR_RXFE != 0 {
            return None;
        }
        Some((self.read_reg(UARTDR) & 0xFF) as u8)
    }

    pub(crate) fn clear_interrupts(&self) {
        self.write_reg(UARTICR, INTERRUPT_CLEAR_ALL);
    }
}
