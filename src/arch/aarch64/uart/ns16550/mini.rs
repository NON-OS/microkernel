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
    /// The mini-UART is switched on as a whole in the AUX block next door,
    /// outside its window; firmware does that when it makes it the console,
    /// so only the transmitter and receiver enables are this side's.
    pub(crate) fn init_mini(&self, line: Line) {
        let r = &self.regs;
        r.write(IER, 0);
        let Line::MiniUart(baud) = line else {
            let cntl = r.read(AUX_CNTL);
            r.write(AUX_CNTL, cntl | CNTL_RX_TX);
            return;
        };
        r.write(AUX_CNTL, 0);
        // 0b11 is eight data bits; the datasheet's single bit gives seven.
        r.write(LCR, LCR_8N1);
        r.write(MCR, 0);
        r.write(FCR, AUX_IIR_CLEAR);
        r.write(AUX_BAUD, baud as u32);
        r.write(AUX_CNTL, CNTL_RX_TX);
    }
}
