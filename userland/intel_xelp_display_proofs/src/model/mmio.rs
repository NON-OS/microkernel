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
//! The model's register reads and writes.

use nonos_intel_display_hw::access::Mmio;

use super::follow::{settle, status_mask};
use super::group;
use super::regs::Regs;

impl Mmio for Regs {
    fn read(&mut self, offset: u32) -> u32 {
        self.reads.push(offset);
        if group::lanes(offset).is_some() {
            return group::JUNK;
        }
        settle(self, offset);
        self.get(offset)
    }
    fn write(&mut self, offset: u32, value: u32) {
        self.writes.push((offset, value));
        let keep = status_mask(offset);
        self.map.insert(offset, (value & !keep) | (self.get(offset) & keep));
        for l in group::lanes(offset).into_iter().flatten() {
            self.map.insert(l, value);
        }
    }
    fn write64(&mut self, offset: u32, value: u64) {
        self.write(offset, value as u32);
    }
    fn read64(&mut self, offset: u32) -> u64 {
        self.read(offset) as u64
    }
}
