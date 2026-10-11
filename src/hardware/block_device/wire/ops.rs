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

//! The requests that carry no data: health, size, flush, and whether the
//! disk can be addressed in sectors at all.

use super::error::WireError;
use super::gate::gate;
use super::header::{OP_FLUSH, OP_HEALTHCHECK};
use super::layout::layout;
use super::lba_map::capacity_sectors;
use super::transport::call;

/// Whether endpoint `index`'s driver answers, asked of its first unit.
pub fn healthcheck(index: u8) -> Result<(), WireError> {
    call(gate(index, 0)?, OP_HEALTHCHECK, &[]).map(|_| ())
}

/// The disk's size in 512-byte sectors, whatever its block size.
pub(super) fn capacity(index: u8, unit: u16) -> Result<u64, WireError> {
    let l = layout(gate(index, unit)?)?;
    capacity_sectors(l.lba_count, l.lba_bytes).ok_or(WireError::BlockSize)
}

pub(super) fn flush(index: u8, unit: u16) -> Result<(), WireError> {
    call(gate(index, unit)?, OP_FLUSH, &[]).map(|_| ())
}

/// False only for a disk whose block size 512-byte sectors cannot map to;
/// the layout has said so on the log.
pub(super) fn sectors_fit(index: u8, unit: u16) -> bool {
    !matches!(gate(index, unit).and_then(layout), Err(WireError::BlockSize))
}
