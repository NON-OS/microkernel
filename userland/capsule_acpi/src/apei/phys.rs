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

//! Firmware's error memory, mapped once per window through the kernel's
//! region rule (reserved, ACPI NVS, ACPI reclaim) and kept: the sources are
//! polled for as long as the capsule runs, and a window asked for again
//! would cost another mapping each time.

use crate::sys::acpi::map;
use alloc::vec::Vec;

const PAGE: u64 = 0x1000;

#[derive(Default)]
pub struct Phys {
    held: Vec<(u64, u64, u64)>,
}

impl Phys {
    fn va(&mut self, addr: u64, len: u64) -> Option<u64> {
        let end = addr.checked_add(len)?;
        let hit =
            |&(b, l, va): &(u64, u64, u64)| (b <= addr && end <= b + l).then(|| va + (addr - b));
        if let Some(va) = self.held.iter().find_map(hit) {
            return Some(va);
        }
        let (start, stop) = (addr & !(PAGE - 1), end.checked_add(PAGE - 1)? & !(PAGE - 1));
        let (va, _grant) = map(start, stop - start).ok()?;
        self.held.push((start, stop - start, va));
        self.held.iter().find_map(hit)
    }

    /// `len` bytes at `addr`, copied out one byte at a time: firmware may be
    /// writing the next record, and a torn wide read could mix two.
    pub fn bytes(&mut self, addr: u64, len: u64) -> Option<Vec<u8>> {
        let va = self.va(addr, len)?;
        // SAFETY: [va, va + len) lies in a mapping the kernel made for this
        // capsule (`va`); volatile, so each byte is read exactly once.
        Some((0..len).map(|i| unsafe { core::ptr::read_volatile((va + i) as *const u8) }).collect())
    }

    pub fn read64(&mut self, addr: u64) -> Option<u64> {
        let b = self.bytes(addr, 8)?;
        Some(u64::from_le_bytes(b.try_into().ok()?))
    }

    /// A 32- or 64-bit write, as GHES's block status and Read Ack need.
    pub fn write(&mut self, addr: u64, width: u64, v: u64) -> bool {
        let Some(va) = self.va(addr, width) else { return false };
        // SAFETY: as for `bytes`; the address is aligned to the width, as
        // the structures that name it require.
        unsafe {
            match width {
                4 => core::ptr::write_volatile(va as *mut u32, v as u32),
                _ => core::ptr::write_volatile(va as *mut u64, v),
            }
        }
        true
    }
}
