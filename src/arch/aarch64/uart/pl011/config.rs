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
use super::super::wait;
use super::constants::*;
use super::device::Pl011;

impl Pl011 {
    pub(crate) fn init(&self, line: Line) {
        let Line::Pl011 { integer, fraction } = line else {
            // Firmware's speed stands; make sure it can send and receive.
            let cr = self.read_reg(UARTCR);
            self.write_reg(UARTCR, cr | CR_UARTEN | CR_TXE | CR_RXE);
            return;
        };
        // The divisors only take while the UART is off, and turning it off
        // mid-byte garbles that byte; firmware may still be draining its
        // last line. A transmitter that never goes idle is reprogrammed
        // anyway, since waiting longer would not make it idle either.
        wait::until(|| self.read_reg(UARTFR) & FR_BUSY == 0);
        self.write_reg(UARTCR, 0);
        self.write_reg(UARTIBRD, integer);
        self.write_reg(UARTFBRD, fraction);
        self.write_reg(UARTLCR_H, LCR_WLEN_8 | LCR_FEN);
        self.write_reg(UARTCR, CR_UARTEN | CR_TXE | CR_RXE);
    }
}
