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

use super::super::line::Line;
use super::constants::*;
use super::device::Ns16550;

impl Ns16550 {
    /// Interrupts go off either way: nothing routes this UART's line yet,
    /// and a receive interrupt nobody takes stays asserted.
    pub(crate) fn init(&self, line: Line) {
        let r = &self.regs;
        r.write(IER, 0);
        let Line::Ns16550(divisor) = line else {
            return;
        };
        r.write(LCR, LCR_DLAB);
        r.write(DLL, (divisor & 0xFF) as u32);
        r.write(DLM, (divisor >> 8) as u32);
        r.write(LCR, LCR_8N1);
        r.write(FCR, FCR_RESET);
        r.write(MCR, MCR_DTR_RTS);
    }
}
