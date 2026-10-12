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

//! The model behind the engine's register trait: registers through the
//! latch rules, GGTT entries straight into the table. The table holds
//! wide entries; Haswell's narrow ones, written 32 bits at a time from
//! 2 MiB, are widened on the way in and narrowed on the way out, so one
//! scanout serves both.

use super::gen9::Gen9;
use crate::engine::access::Mmio;

const NARROW: std::ops::Range<u32> = 0x20_0000..0x40_0000;
const WIDE: u32 = 0x80_0000;

fn widen(v: u32) -> u64 {
    let addr = (v as u64 & 0xFFFF_F000) | ((v as u64 & 0xFF0) << 28);
    addr | (v as u64 & 1)
}

fn narrow(v: u64) -> u32 {
    ((v & 0xFFFF_F000) | ((v >> 28) & 0xFF0) | (v & 1)) as u32
}

impl Mmio for Gen9 {
    fn read(&mut self, off: u32) -> u32 {
        match NARROW.contains(&off) {
            true => narrow(self.ggtt[((off - NARROW.start) / 4) as usize]),
            false => self.on_read(off),
        }
    }
    fn write(&mut self, off: u32, v: u32) {
        match NARROW.contains(&off) {
            true => self.ggtt[((off - NARROW.start) / 4) as usize] = widen(v),
            false => self.on_write(off, v),
        }
    }
    fn write64(&mut self, off: u32, v: u64) {
        self.ggtt[((off - WIDE) / 8) as usize] = v;
    }
    fn read64(&mut self, off: u32) -> u64 {
        self.ggtt[((off - WIDE) / 8) as usize]
    }
}
