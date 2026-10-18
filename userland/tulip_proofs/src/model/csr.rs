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

//! Writes to the CSRs, as the manuals and QEMU's tulip_write take them: a
//! software reset, the poll demands, the ring addresses, the status bits a
//! one clears, the operation mode starting and stopping each process. A
//! write to any other CSR is a driver touching what it should not.

use super::events::Event;
use super::part::{Part, RS_STOPPED, RS_SUSPENDED, RS_WAITING, TS_STOPPED, TS_SUSPENDED};

const SR: u32 = 1 << 1;
const ST: u32 = 1 << 13;

impl Part {
    pub(super) fn write_csr(&mut self, n: usize, v: u32) {
        self.log.push(Event::Csr(n as u16, v));
        match n {
            0 if v & 1 != 0 => self.reset(),
            0 => self.csr[0] = v,
            1 => self.transmit(),
            2 if self.rs == RS_SUSPENDED => self.rs = RS_WAITING,
            2 => {}
            3 => (self.csr[3], self.rx_at) = (v & !3, v & !3),
            4 => (self.csr[4], self.tx_at) = (v & !3, v & !3),
            5 => self.csr[5] &= !(v & 0x0C01_FFFF),
            6 => self.operation_mode(v),
            7 => self.csr[7] = v,
            _ => panic!("driver wrote {v:#x} to CSR{n}"),
        }
    }

    fn operation_mode(&mut self, v: u32) {
        self.csr[6] = v;
        let start = !self.faults.never_starts;
        self.rs = if v & SR != 0 && start { RS_WAITING } else { RS_STOPPED };
        self.ts = if v & ST != 0 && start { TS_SUSPENDED } else { TS_STOPPED };
        if self.ts != TS_STOPPED {
            self.transmit();
        }
    }
    /// QEMU's tulip_reset: CSR6 comes back with promiscuous mode set, which
    /// a driver must clear rather than OR into.
    pub(super) fn reset_values(&mut self) {
        self.csr = [0; 16];
        (self.csr[0], self.csr[5], self.csr[6], self.csr[7]) =
            (0xFE00_0000, 0xF000_0000, 0x3200_0040, 0xF3FE_0000);
        (self.rs, self.ts, self.rx_at, self.tx_at) = (RS_STOPPED, TS_STOPPED, 0, 0);
        self.filter = [[0; 6]; 16];
    }

    pub(super) fn reset(&mut self) {
        let grants_back = nonos_libc::given_back().len();
        self.log.push(Event::Reset { grants_back });
        self.reset_values();
        if self.faults.stuck_in_reset {
            self.csr[0] |= 1;
        }
    }
}
