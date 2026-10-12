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

//! Writing one 32-bit entry of a device's remapping table. Two entries share
//! a quadword, so the other half is written back as it was; the caller holds
//! the table lock, so nothing changes it between the read and the write.

use super::super::error::AmdViError;
use crate::arch::x86_64::iommu::tables::view::Frame;

pub(super) fn write_irte(table_phys: u64, index: u16, entry: u32) -> Result<(), AmdViError> {
    let words = Frame::at(table_phys).map_err(|_| AmdViError::TableUnreachable)?;
    let at = index as usize / 2;
    let shift = (index as u32 % 2) * 32;
    let word = words.read(at)?;
    let kept = word & !(0xFFFF_FFFFu64 << shift);
    words.write(at, kept | ((entry as u64) << shift))?;
    Ok(())
}
