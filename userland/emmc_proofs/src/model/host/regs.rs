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

//! The register file: reset to what the host reports, read and written.

use super::sim::Sim;

impl Sim {
    pub(super) fn now(&self) -> u64 {
        self.now.get()
    }

    pub(super) fn reset_all(&mut self) {
        self.regs = [0; 256];
        self.regs[0x40..0x44].copy_from_slice(&self.cfg.caps.to_le_bytes());
        self.regs[0x44..0x48].copy_from_slice(&self.cfg.caps1.to_le_bytes());
        self.regs[0xfe..0x100].copy_from_slice(&self.cfg.version.to_le_bytes());
        self.int = 0;
        self.tc_at = None;
        self.dev().power(false);
    }

    pub(super) fn get(&self, off: u32, n: usize) -> u32 {
        let mut v = 0u32;
        for i in 0..n {
            v |= (self.regs[off as usize + i] as u32) << (8 * i);
        }
        v
    }

    pub(super) fn put(&mut self, off: u32, n: usize, v: u32) {
        for i in 0..n {
            self.regs[off as usize + i] = (v >> (8 * i)) as u8;
        }
    }

    pub(super) fn violate(&mut self, s: String) {
        self.violations.push(s);
    }
}
