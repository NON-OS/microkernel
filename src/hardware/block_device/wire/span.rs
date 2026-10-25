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

//! A sector request mapped onto the disk's blocks, refused whole when any
//! part of it falls past the end. Pure, so kernel_proofs runs it on the host.

use super::error::WireError;
use super::lba_map::{plan, Plan, SECTOR};

pub(crate) fn span(
    lba_bytes: u32,
    lba_count: u64,
    sector: u64,
    bytes: usize,
) -> Result<Plan, WireError> {
    if bytes == 0 || !bytes.is_multiple_of(SECTOR) {
        return Err(WireError::InvalidArgument);
    }
    let p = plan(sector, bytes, lba_bytes).ok_or(WireError::OutOfRange)?;
    let last = [p.head.map(|h| h.lba), p.body.map(|w| w.lba + (w.lbas - 1)), p.tail.map(|t| t.lba)];
    if last.iter().flatten().any(|&lba| lba >= lba_count) {
        return Err(WireError::OutOfRange);
    }
    Ok(p)
}
