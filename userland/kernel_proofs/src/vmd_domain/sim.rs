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

//! A PCI bus behind a VMD: functions hang off bridges, are reachable only on
//! the bus their bridge's secondary number names, and size their BARs.

use super::domain::ConfigPort;

pub struct Func {
    pub parent: Option<usize>,
    pub device: u8,
    pub function: u8,
    pub regs: [u32; 64],
    /// Per BAR register: (size, 64-bit). Size 0 is no BAR.
    pub bars: [(u64, bool); 6],
}

impl Func {
    pub fn endpoint(parent: Option<usize>, device: u8, id: u32, bars: [(u64, bool); 6]) -> Self {
        let mut regs = [0u32; 64];
        regs[0] = id;
        regs[1] = 0x0010_0006; // capabilities list, memory and master on
        regs[3] = 0;
        for (i, (size, wide)) in bars.iter().enumerate() {
            if *size != 0 {
                regs[4 + i] = if *wide { 0x4 } else { 0x0 };
            }
        }
        Self { parent, device, function: 0, regs, bars }
    }

    pub fn bridge(parent: Option<usize>, device: u8) -> Self {
        let mut regs = [0u32; 64];
        regs[0] = 0x9a09_8086;
        regs[3] = 0x0001_0000;
        Self { parent, device, function: 0, regs, bars: [(0, false); 6] }
    }

    pub fn reg(&self, offset: u16) -> u32 {
        self.regs[offset as usize / 4]
    }
}

pub struct Sim {
    pub root_bus: u8,
    pub funcs: Vec<Func>,
    pub writes_outside: usize,
}

impl Sim {
    fn bus_of(&self, index: usize) -> Option<u8> {
        match self.funcs[index].parent {
            None => Some(self.root_bus),
            Some(p) => {
                self.bus_of(p)?;
                let secondary = ((self.funcs[p].regs[6] >> 8) & 0xFF) as u8;
                if secondary == 0 {
                    None
                } else {
                    Some(secondary)
                }
            }
        }
    }

    fn find(&self, bus: u8, device: u8, function: u8) -> Option<usize> {
        (0..self.funcs.len()).find(|&i| {
            let f = &self.funcs[i];
            f.device == device && f.function == function && self.bus_of(i) == Some(bus)
        })
    }

    pub fn at(&self, bus: u8, device: u8, function: u8) -> Option<&Func> {
        self.find(bus, device, function).map(|i| &self.funcs[i])
    }
}

impl ConfigPort for Sim {
    fn read32(&mut self, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        match self.find(bus, device, function) {
            Some(i) => self.funcs[i].regs[offset as usize / 4],
            None => 0xFFFF_FFFF,
        }
    }

    fn write32(&mut self, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        let Some(i) = self.find(bus, device, function) else {
            self.writes_outside += 1;
            return;
        };
        let f = &mut self.funcs[i];
        let slot = offset as usize / 4;
        if (4..10).contains(&slot) && f.regs[3] & 0x007F_0000 == 0 {
            let bar = slot - 4;
            let (size, wide) = f.bars[bar];
            let high_of = bar > 0 && f.bars[bar - 1].1;
            if high_of {
                let size = f.bars[bar - 1].0;
                let mask = !(size - 1) >> 32;
                f.regs[slot] = value & mask as u32;
            } else if size == 0 {
                f.regs[slot] = 0;
            } else {
                let mask = (!(size - 1)) as u32 & !0xF;
                f.regs[slot] = (value & mask) | if wide { 0x4 } else { 0 };
            }
            return;
        }
        f.regs[slot] = value;
    }
}
