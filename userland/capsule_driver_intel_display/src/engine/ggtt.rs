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

//! Writing the global GTT, the table the display engine translates a
//! plane's graphics address through, in the generation's entry format
//! (`Pte`). A Haswell entry is one 32-bit write, a later one 64-bit.

use super::access::Mmio;
use super::error::Error;
use super::ggtt_pte::Pte;

/// GFX_FLSH_CNTL: a write makes the engine drop cached GGTT entries
/// (offset as the PRM names it; unverified on each generation).
pub const GFX_FLSH_CNTL: u32 = 0x10_1008;

fn put(m: &mut dyn Mmio, t: Pte, i: u32, v: u64) {
    let at = t.table() + i * t.size();
    match t {
        Pte::Narrow => m.write(at, v as u32),
        Pte::Wide => m.write64(at, v),
    }
}

/// Point entries `first..` at the pages `iovas` (4 KiB each, aligned),
/// then post the writes and flush, so a flip after this sees them.
pub fn map(m: &mut dyn Mmio, t: Pte, first: u32, iovas: &[u64], total: u32) -> Result<(), Error> {
    let end = first as u64 + iovas.len() as u64;
    if end > total as u64 || iovas.iter().any(|&a| a & 0xFFF != 0 || a >= t.reach()) {
        return Err(Error::Argument);
    }
    for (i, &a) in iovas.iter().enumerate() {
        put(m, t, first + i as u32, t.encode(a));
    }
    flush(m, t, first + iovas.len() as u32 - 1);
    Ok(())
}

/// Mark `count` entries from `first` not present.
pub fn unmap(m: &mut dyn Mmio, t: Pte, first: u32, count: u32, total: u32) -> Result<(), Error> {
    if first as u64 + count as u64 > total as u64 || count == 0 {
        return Err(Error::Argument);
    }
    for i in first..first + count {
        put(m, t, i, 0);
    }
    flush(m, t, first + count - 1);
    Ok(())
}

/// Read the last entry written back, so the writes have landed, then
/// flush the engine's cached translations.
fn flush(m: &mut dyn Mmio, t: Pte, last: u32) {
    let at = t.table() + last * t.size();
    let _ = match t {
        Pte::Narrow => m.read(at) as u64,
        Pte::Wide => m.read64(at),
    };
    m.write(GFX_FLSH_CNTL, 1);
    let _ = m.read(GFX_FLSH_CNTL);
}
