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
//! Registers as a map with a log of every write. TRANS_CONF's state bit
//! (30, Vol 2c p. 1341) follows its enable bit (31) only after `lag` reads
//! of that register, or never when `lag` is None.

use nonos_intel_display_hw::access::Mmio;
use std::collections::BTreeMap;

/// TRANS_CONF A to D (Vol 2c p. 1340), stated here, not taken from the crate.
pub const CONFS: [u32; 4] = [0x70008, 0x71008, 0x72008, 0x73008];

pub struct Regs {
    pub map: BTreeMap<u32, u32>,
    pub writes: Vec<(u32, u32)>,
    pub reads: Vec<u32>,
    pub lag: Option<u32>,
    pub(super) pending: BTreeMap<u32, u32>,
}

impl Regs {
    pub fn new(lag: Option<u32>) -> Self {
        Regs { map: BTreeMap::new(), writes: vec![], reads: vec![], lag, pending: BTreeMap::new() }
    }

    pub fn get(&self, reg: u32) -> u32 {
        self.map.get(&reg).copied().unwrap_or(0)
    }

    pub fn set(&mut self, reg: u32, v: u32) {
        self.map.insert(reg, v);
    }
}

impl Mmio for Regs {
    fn read(&mut self, offset: u32) -> u32 {
        self.reads.push(offset);
        if CONFS.contains(&offset) {
            super::conf::settle(self, offset);
        }
        self.get(offset)
    }
    fn write(&mut self, offset: u32, value: u32) {
        self.writes.push((offset, value));
        self.pending.remove(&offset);
        // The state bit is the hardware's; a write cannot set it.
        let keep = if CONFS.contains(&offset) { self.get(offset) & (1 << 30) } else { 0 };
        let v = if CONFS.contains(&offset) { (value & !(1 << 30)) | keep } else { value };
        self.map.insert(offset, v);
    }
    fn write64(&mut self, offset: u32, value: u64) {
        self.write(offset, value as u32);
    }
    fn read64(&mut self, offset: u32) -> u64 {
        self.read(offset) as u64
    }
}
