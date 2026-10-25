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

//! One SystemMemory access at a mapped address: once, at the width asked,
//! never split or merged. A region's units are aligned to its base, which
//! need not be; an access that lands unaligned is made a byte at a time,
//! the one case where firmware's single access becomes several.

fn aligned(va: u64, width: u32) -> bool {
    matches!(width, 1 | 2 | 4 | 8) && va.is_multiple_of(width as u64)
}

pub fn read(va: u64, width: u32) -> Option<u64> {
    if !aligned(va, width) {
        return matches!(width, 2 | 4 | 8).then(|| {
            (0..width as u64).rev().fold(0, |v, i| (v << 8) | read(va + i, 1).unwrap_or(0))
        });
    }
    // SAFETY: `va` lies in a mapping the kernel made for this capsule
    // (`Windows::va`), aligned to the width; volatile so the access happens
    // exactly once.
    unsafe {
        Some(match width {
            1 => core::ptr::read_volatile(va as *const u8) as u64,
            2 => core::ptr::read_volatile(va as *const u16) as u64,
            4 => core::ptr::read_volatile(va as *const u32) as u64,
            _ => core::ptr::read_volatile(va as *const u64),
        })
    }
}

pub fn write(va: u64, width: u32, v: u64) -> bool {
    if !aligned(va, width) {
        return matches!(width, 2 | 4 | 8)
            && (0..width as u64).all(|i| write(va + i, 1, v >> (8 * i)));
    }
    // SAFETY: as for `read`.
    unsafe {
        match width {
            1 => core::ptr::write_volatile(va as *mut u8, v as u8),
            2 => core::ptr::write_volatile(va as *mut u16, v as u16),
            4 => core::ptr::write_volatile(va as *mut u32, v as u32),
            _ => core::ptr::write_volatile(va as *mut u64, v),
        }
    }
    true
}
