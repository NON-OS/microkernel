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

//! The fourth step of a map: a user VA window in the per-capsule MMIO region,
//! and the pages installed there user, read and write, uncached, no-execute.

use super::types::MmioMapError;
use super::window::{Window, PAGE_SIZE};
use crate::hardware::broker::grant::{self, USER_MMIO_BASE, USER_MMIO_END};
use crate::memory::addr::{PhysAddr, VirtAddr};

pub(super) fn install(w: &Window) -> Result<VirtAddr, MmioMapError> {
    let pages = w.page_bytes / PAGE_SIZE;
    let user_va = grant::reserve_user_va(pages).ok_or(MmioMapError::NoVaSpace)?;
    let user_va_end = user_va.as_u64().checked_add(w.page_bytes).ok_or(MmioMapError::Overflow)?;
    if user_va.as_u64() < USER_MMIO_BASE || user_va_end > USER_MMIO_END {
        return Err(MmioMapError::NoVaSpace);
    }
    let phys = PhysAddr::new(w.page_start);
    if crate::memory::paging::map_user_mmio(user_va, phys, w.page_bytes as usize).is_err() {
        return Err(MmioMapError::MapFailed);
    }
    Ok(user_va)
}
