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

//! Which BAR slot a `MkMmioMap` request may name: one inside the record's
//! `bar_count`, of the MMIO kind, with a size. Anything else in a slot is
//! no window, whatever bytes it holds. Pure: `kernel_proofs` mounts it.

use super::super::device::{Bar, BAR_KIND_MMIO};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarRefusal {
    /// Past the record's windows.
    BadIndex,
    /// A slot that is not a memory window, or holds no bytes.
    NotMmio,
}

pub fn mappable(bars: &[Bar], bar_count: u8, index: u8) -> Result<Bar, BarRefusal> {
    let i = index as usize;
    if i >= bars.len() || i >= bar_count as usize {
        return Err(BarRefusal::BadIndex);
    }
    let bar = bars[i];
    if bar.kind != BAR_KIND_MMIO || bar.size == 0 {
        return Err(BarRefusal::NotMmio);
    }
    Ok(bar)
}
