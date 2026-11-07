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

use crate::memory::addr::PhysAddr;

use super::pte_flags;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn new(phys_addr: PhysAddr, flags: u64) -> Self {
        Self((phys_addr.as_u64() & pte_flags::ADDR_MASK) | flags)
    }

    pub fn is_present(&self) -> bool {
        self.0 & pte_flags::PRESENT != 0
    }

    pub fn is_writable(&self) -> bool {
        self.0 & pte_flags::WRITABLE != 0
    }

    pub fn is_user_accessible(&self) -> bool {
        self.0 & pte_flags::USER_ACCESSIBLE != 0
    }

    pub fn is_huge_page(&self) -> bool {
        self.0 & pte_flags::HUGE_PAGE != 0
    }

    pub fn phys_addr(&self) -> PhysAddr {
        PhysAddr::new(self.0 & pte_flags::ADDR_MASK)
    }

    pub fn flags(&self) -> u64 {
        self.0 & !pte_flags::ADDR_MASK
    }

    pub fn set_flags(&mut self, flags: u64) {
        self.0 = (self.0 & pte_flags::ADDR_MASK) | flags;
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PTE(0x{:016X})", self.0)
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    pub(crate) entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        const EMPTY: PageTableEntry = PageTableEntry::empty();
        Self { entries: [EMPTY; 512] }
    }

    pub fn entry(&self, index: usize) -> &PageTableEntry {
        &self.entries[index]
    }

    pub fn entry_mut(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self.entries[index]
    }

    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.clear();
        }
    }

    pub fn copy_from(&mut self, other: &PageTable) {
        for i in 0..512 {
            self.entries[i] = other.entries[i];
        }
    }

    pub fn copy_kernel_entries(&mut self, other: &PageTable) {
        for i in 256..512 {
            self.entries[i] = other.entries[i];
        }
    }
}
