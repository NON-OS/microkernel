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

//! The part's registers, its two processes and what it has done.

use super::events::{Event, Faults};
use super::memory::Memory;

pub const RS_STOPPED: u32 = 0;
pub const RS_WAITING: u32 = 3;
pub const RS_SUSPENDED: u32 = 4;
pub const TS_STOPPED: u32 = 0;
pub const TS_SUSPENDED: u32 = 6;

pub struct Part {
    pub csr: [u32; 16],
    pub rs: u32,
    pub ts: u32,
    pub rx_at: u32,
    pub tx_at: u32,
    pub filter: [[u8; 6]; 16],
    pub mem: Memory,
    pub faults: Faults,
    pub log: Vec<Event>,
    pub sent: Vec<Vec<u8>>,
}

impl Part {
    /// A part out of power-on reset: both processes stopped, an empty filter.
    pub fn new(mem: Memory) -> Self {
        let mut p = Self {
            csr: [0; 16],
            rs: RS_STOPPED,
            ts: TS_STOPPED,
            rx_at: 0,
            tx_at: 0,
            filter: [[0; 6]; 16],
            mem,
            faults: Faults::default(),
            log: Vec::new(),
            sent: Vec::new(),
        };
        p.reset_values();
        p
    }

    /// CSR5 as read: the status bits with both process states.
    pub fn csr5(&self) -> u32 {
        (self.csr[5] & !(0x3F << 17)) | self.rs << 17 | self.ts << 20
    }
}
