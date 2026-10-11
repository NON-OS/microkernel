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

//! The global GTT: the table the display engine translates a plane's
//! graphics address through. On Gen9 its 64-bit entries fill the upper
//! half of GTTMMADR, from 8 MiB (offset unverified against the PRM's
//! GTTMMADR description). An entry is a page address in bits 38:12 and a
//! present bit 0; the address is the page's IOVA in the capsule's IOMMU
//! domain, since the engine's reads pass through the graphics DMAR unit.

use super::access::Mmio;
use super::error::Error;

pub const GGTT_OFFSET: u32 = 0x80_0000;
/// GFX_FLSH_CNTL: a write makes the engine drop cached GGTT entries
/// (offset as the PRM names it; unverified on Gen9).
pub const GFX_FLSH_CNTL: u32 = 0x10_1008;
const ADDR_MASK: u64 = 0x7F_FFFF_F000;

pub fn pte(iova: u64) -> u64 {
    (iova & ADDR_MASK) | 1
}

/// Point entries `first..` at the pages `iovas` (4 KiB each, aligned),
/// then post the writes and flush, so a flip after this sees them.
pub fn map(m: &mut dyn Mmio, first: u32, iovas: &[u64], total: u32) -> Result<(), Error> {
    let end = first as u64 + iovas.len() as u64;
    if end > total as u64 || iovas.iter().any(|a| a & 0xFFF != 0 || a & !ADDR_MASK != 0) {
        return Err(Error::Argument);
    }
    for (i, &a) in iovas.iter().enumerate() {
        m.write64(GGTT_OFFSET + (first + i as u32) * 8, pte(a));
    }
    flush(m, first + iovas.len() as u32 - 1);
    Ok(())
}

/// Mark `count` entries from `first` not present.
pub fn unmap(m: &mut dyn Mmio, first: u32, count: u32, total: u32) -> Result<(), Error> {
    if first as u64 + count as u64 > total as u64 || count == 0 {
        return Err(Error::Argument);
    }
    for i in first..first + count {
        m.write64(GGTT_OFFSET + i * 8, 0);
    }
    flush(m, first + count - 1);
    Ok(())
}

/// Read the last entry written back, so the writes have landed, then
/// flush the engine's cached translations.
fn flush(m: &mut dyn Mmio, last: u32) {
    let _ = m.read64(GGTT_OFFSET + last * 8);
    m.write(GFX_FLSH_CNTL, 1);
    let _ = m.read(GFX_FLSH_CNTL);
}
