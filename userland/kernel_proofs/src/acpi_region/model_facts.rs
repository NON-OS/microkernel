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

//! The model machine answering the rule's questions.

use super::machine::Machine;
use super::rule::{Facts, Hardware, MemKind};

fn overlaps(a: u64, b: u64, lo: u64, hi: u64) -> bool {
    a < hi && lo < b
}

impl Facts for Machine {
    fn mem_kinds(&self, base: u64, end: u64, each: &mut dyn FnMut(MemKind)) {
        let mut covered = 0u64;
        for (lo, hi, k) in &self.map {
            if overlaps(base, end, *lo, *hi) {
                each(*k);
                covered += (*hi).min(end) - (*lo).max(base);
            }
        }
        if covered < end - base {
            each(MemKind::Unmapped);
        }
    }
    fn mmio_claimed(&self, base: u64, end: u64) -> bool {
        self.windows.iter().any(|(lo, hi)| overlaps(base, end, *lo, *hi))
    }
    fn protected(&self, base: u64, end: u64) -> Option<Hardware> {
        self.hardware.iter().find(|(lo, hi, _)| overlaps(base, end, *lo, *hi)).map(|h| h.2)
    }
    fn reset_port(&self, base: u64, end: u64) -> bool {
        self.reset.iter().any(|(lo, hi)| overlaps(base, end, *lo, *hi))
    }
    fn ports_claimed(&self, base: u16, count: u32) -> bool {
        let end = base as u64 + count as u64;
        self.ports.iter().any(|(lo, hi)| overlaps(base as u64, end, *lo as u64, *hi as u64))
    }
    fn pci_claimed(&self, bdf: (u16, u8, u8, u8)) -> bool {
        self.pci.contains(&bdf)
    }
}
