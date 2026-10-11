// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use alloc::vec;

use super::error::WireError;
use super::gate::gate;
use super::layout::layout;
use super::lba_map::{chunks, Partial};
use super::native::read_lbas;
use super::span::span;
use super::target::Target;

/// Read `out.len()` bytes from 512-byte sector `sector` of unit `unit` behind
/// endpoint `index`, cut into requests the driver takes. A part of a block at
/// either end is read whole and the covered bytes copied out.
pub(super) fn read_blocks(
    index: u8,
    unit: u16,
    sector: u64,
    out: &mut [u8],
) -> Result<(), WireError> {
    let t = gate(index, unit)?;
    let layout = layout(t)?;
    let plan = span(layout.lba_bytes, layout.lba_count, sector, out.len())?;
    let lba_bytes = layout.lba_bytes as usize;
    if let Some(head) = plan.head {
        read_partial(t, head, lba_bytes, out)?;
    }
    if let Some(whole) = plan.body {
        for (lba, n, offset) in chunks(whole, layout.per_request, layout.lba_bytes) {
            let at = whole.at + offset;
            read_lbas(t, lba, n, &mut out[at..at + n as usize * lba_bytes])?;
        }
    }
    if let Some(tail) = plan.tail {
        read_partial(t, tail, lba_bytes, out)?;
    }
    Ok(())
}

fn read_partial(t: Target, p: Partial, lba_bytes: usize, out: &mut [u8]) -> Result<(), WireError> {
    let mut block = vec![0u8; lba_bytes];
    read_lbas(t, p.lba, 1, &mut block)?;
    out[p.at..p.at + p.len].copy_from_slice(&block[p.offset..p.offset + p.len]);
    Ok(())
}
