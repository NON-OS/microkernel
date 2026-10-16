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

//! A per-CPU block found again by the address its CPU's register holds, as
//! the aarch64 vectors find it through TPIDR_EL1: by its index in the one
//! array of blocks, so no number is taken for a pointer.

use core::mem::size_of;

use super::operations::get;
use super::types::PerCpuData;

/// The block at `base`, if `base` is exactly one of them.
pub fn by_base(base: u64) -> Option<&'static PerCpuData> {
    let first = get(0)? as *const PerCpuData as u64;
    let offset = base.checked_sub(first)?;
    let stride = size_of::<PerCpuData>() as u64;
    if offset % stride != 0 {
        return None;
    }
    get(usize::try_from(offset / stride).ok()?)
}
