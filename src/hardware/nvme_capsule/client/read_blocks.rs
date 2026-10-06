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

use alloc::vec;

use super::super::capability::gate_call;
use super::super::error::DriverNvmeError;
use super::layout::layout;
use super::lba_map::{chunks, plan, Partial, SECTOR};
use super::native::read_lbas;

/// Read `out.len()` bytes from 512-byte sector `sector`. The request is
/// mapped onto the namespace's LBAs and cut into commands the capsule takes
/// (its MDTS and data buffer); a part of an LBA at either end is read whole
/// and the covered bytes copied out.
pub fn read_blocks(sector: u64, out: &mut [u8]) -> Result<(), DriverNvmeError> {
    let _caller = gate_call()?;
    if out.is_empty() || !out.len().is_multiple_of(SECTOR) {
        return Err(DriverNvmeError::InvalidArgument);
    }
    let layout = layout()?;
    let plan = plan(sector, out.len(), layout.lba_size).ok_or(DriverNvmeError::OutOfRange)?;
    let lba_bytes = layout.lba_size as usize;
    if let Some(head) = plan.head {
        read_partial(head, lba_bytes, out)?;
    }
    if let Some(whole) = plan.body {
        for (lba, n, offset) in chunks(whole, layout.per_request, layout.lba_size) {
            let at = whole.at + offset;
            read_lbas(lba, n, &mut out[at..at + n as usize * lba_bytes])?;
        }
    }
    if let Some(tail) = plan.tail {
        read_partial(tail, lba_bytes, out)?;
    }
    Ok(())
}

fn read_partial(p: Partial, lba_bytes: usize, out: &mut [u8]) -> Result<(), DriverNvmeError> {
    let mut block = vec![0u8; lba_bytes];
    read_lbas(p.lba, 1, &mut block)?;
    out[p.at..p.at + p.len].copy_from_slice(&block[p.offset..p.offset + p.len]);
    Ok(())
}
