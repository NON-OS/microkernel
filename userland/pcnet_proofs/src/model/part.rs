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

//! The part's registers and what it has done. Anything below 0x10 in the
//! window is the APROM, which this driver must never read: its address is
//! the factory one the driver replaces.

use super::events::{Event, Faults, Rings};
use super::memory::Memory;

pub const STOP: u16 = 1 << 2;

pub struct Part {
    pub rap: u16,
    pub csr: [u16; 128],
    pub bcr: [u16; 128],
    pub mem: Memory,
    pub faults: Faults,
    /// CSR88 | CSR89 << 16; QEMU's and VirtualBox's 79C970A by default.
    pub chip_id: u32,
    pub link: bool,
    pub log: Vec<Event>,
    pub sent: Vec<Vec<u8>>,
    pub ready: bool,
    pub rings: Rings,
}

impl Part {
    /// A part out of power-on reset: stopped, word I/O, link up.
    pub fn new(mem: Memory) -> Self {
        let (mut csr, bcr) = ([0u16; 128], [0u16; 128]);
        csr[0] = STOP;
        let faults = Faults::default();
        let (log, sent, rings) = (Vec::new(), Vec::new(), Rings::default());
        Self {
            rap: 0,
            csr,
            bcr,
            mem,
            faults,
            chip_id: 0x0262_1003,
            link: true,
            log,
            sent,
            ready: false,
            rings,
        }
    }

    pub(super) fn reset(&mut self) {
        let grants_back = nonos_libc::given_back().len();
        self.log.push(Event::Reset { grants_back });
        (self.rap, self.bcr[20], self.csr[3], self.ready) = (0, 0, 0, false);
        self.csr[0] = if self.faults.stuck_in_reset { 0 } else { STOP };
    }
}
