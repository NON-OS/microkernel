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

//! SystemMemory: each OperationRegion asked for whole the first time it is
//! touched, so the kernel logs one grant per region; a region the rule
//! refuses whole falls back to the pages the access needs, so a region that
//! strays over a register the kernel keeps still serves its other fields.

use crate::sys::acpi::map;
use alloc::vec::Vec;

const PAGE: u64 = 0x1000;

#[derive(Default)]
pub struct Windows {
    /// (physical base, length, where base is mapped)
    held: Vec<(u64, u64, u64)>,
}

impl Windows {
    fn find(&self, addr: u64, width: u64) -> Option<u64> {
        let end = addr.checked_add(width)?;
        self.held
            .iter()
            .find(|(b, l, _)| *b <= addr && end <= b.saturating_add(*l))
            .map(|(b, _, va)| va + (addr - b))
    }

    fn ask(&mut self, base: u64, len: u64) -> bool {
        match map(base, len) {
            Ok((va, _grant)) => {
                self.held.push((base, len, va));
                true
            }
            Err(_) => false,
        }
    }

    /// Where `width` bytes at `addr`, in the region [base, base + len), are
    /// mapped in this capsule; None if neither the region nor its pages are
    /// granted.
    pub fn va(&mut self, addr: u64, width: u64, region: (u64, u64)) -> Option<u64> {
        if let Some(va) = self.find(addr, width) {
            return Some(va);
        }
        if !self.ask(region.0, region.1) {
            let start = addr & !(PAGE - 1);
            let end = addr.checked_add(width)?.checked_add(PAGE - 1)? & !(PAGE - 1);
            self.ask(start, end - start);
        }
        self.find(addr, width)
    }
}
