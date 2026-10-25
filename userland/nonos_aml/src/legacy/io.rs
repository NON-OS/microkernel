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

//! The I/O and fixed memory ranges a legacy device's _CRS names: the
//! small I/O Port descriptor (ACPI 6.5, 6.4.2.5), the small Fixed Location
//! I/O Port descriptor (6.4.2.6), and the large 32-bit Fixed Memory Range
//! descriptor (6.4.3.4), where an HPET's registers are.

use crate::platform::items;
use alloc::vec::Vec;

const IO_PORT: u8 = 0x08;
const FIXED_IO: u8 = 0x09;
const MEMORY32_FIXED: u8 = 0x06;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub base: u64,
    pub len: u64,
}

/// I/O ranges in order. An I/O Port descriptor whose minimum and maximum
/// differ is relocatable; its minimum is where the device sits until it is
/// moved, which NONOS never does. A zero-length range is left out.
pub fn io_ranges(t: &[u8]) -> Vec<Range> {
    let mut out = Vec::new();
    for i in items(t).iter().filter(|i| !i.large) {
        let b = i.body;
        let r = match i.kind {
            IO_PORT if b.len() >= 7 => {
                Range { base: u64::from(u16::from_le_bytes([b[1], b[2]])), len: u64::from(b[6]) }
            }
            // Ten address bits, as the ISA bus decoded them.
            FIXED_IO if b.len() >= 3 => Range {
                base: u64::from(u16::from_le_bytes([b[0], b[1]]) & 0x3FF),
                len: u64::from(b[2]),
            },
            _ => continue,
        };
        if r.len != 0 {
            out.push(r);
        }
    }
    out
}

pub fn memory_ranges(t: &[u8]) -> Vec<Range> {
    let fixed = |b: &[u8]| Range {
        base: u64::from(u32::from_le_bytes([b[1], b[2], b[3], b[4]])),
        len: u64::from(u32::from_le_bytes([b[5], b[6], b[7], b[8]])),
    };
    let ours =
        items(t).into_iter().filter(|i| i.large && i.kind == MEMORY32_FIXED && i.body.len() >= 9);
    ours.map(|i| fixed(i.body)).filter(|r| r.len != 0).collect()
}
