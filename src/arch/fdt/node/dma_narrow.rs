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

//! One bus's `dma-ranges` applied to the window found so far.

use crate::arch::fdt::endian::be_cells;
use crate::arch::fdt::error::FdtError;

use super::cells::Cells;
use super::dma_window::DmaWindow;

/// `w` maps device bus addresses to this bus's child space (its `cpu_base`
/// is an address there, not yet the CPU's); the result maps them to the
/// parent's. Entries are (child address, parent address, size), as `ranges`.
/// Of the entries the window overlaps, the widest is the one RAM is behind;
/// the narrow ones a Pi lists are its own registers, for its DMA engines.
pub(super) fn narrow(
    data: &[u8],
    own: Cells,
    parent: Cells,
    w: DmaWindow,
) -> Result<Option<DmaWindow>, FdtError> {
    if data.is_empty() {
        return Ok(Some(w));
    }
    own.fit_u64()?;
    parent.fit_u64()?;
    let parent_at = own.address as usize * 4;
    let size_at = parent_at + parent.address as usize * 4;
    let stride = size_at + own.size as usize * 4;
    if !data.len().is_multiple_of(stride) {
        return Err(FdtError::BadCells);
    }
    let mut best: Option<DmaWindow> = None;
    for entry in data.chunks_exact(stride) {
        let child = be_cells(entry, 0, own.address)?;
        let base = be_cells(entry, parent_at, parent.address)?;
        let span = be_cells(entry, size_at, own.size)?;
        let (Some(child_end), Some(_)) = (child.checked_add(span), base.checked_add(span)) else {
            continue;
        };
        let lo = w.cpu_base.max(child);
        let hi = w.cpu_base.saturating_add(w.size).min(child_end);
        if hi > lo && best.is_none_or(|b| hi - lo > b.size) {
            let size = hi - lo;
            best = Some(DmaWindow {
                bus_base: w.bus_base + (lo - w.cpu_base),
                cpu_base: base + (lo - child),
                size,
            });
        }
    }
    Ok(best)
}
