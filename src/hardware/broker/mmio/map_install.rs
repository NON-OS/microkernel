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

//! The fourth step of a map: a user VA window in the holder's MMIO region,
//! and the pages installed there user, read and write, no-execute, uncached
//! or write-combining as `memory_type` chose.

use super::memory_type::MemType;
use super::types::MmioMapError;
use super::window::{Window, PAGE_SIZE};
use crate::hardware::broker::grant::{self, USER_MMIO_BASE, USER_MMIO_END};
use crate::memory::addr::{PhysAddr, VirtAddr};

/// From `pid`'s own window, given back on every refusal here.
pub(super) fn install(pid: u32, w: &Window, kind: MemType) -> Result<VirtAddr, MmioMapError> {
    let pages = w.page_bytes / PAGE_SIZE;
    let user_va = grant::reserve_user_va(pid, pages).ok_or(MmioMapError::NoVaSpace)?;
    let (va, bytes) = (user_va.as_u64(), w.page_bytes);
    if va < USER_MMIO_BASE || va.checked_add(bytes).is_none_or(|end| end > USER_MMIO_END) {
        grant::release_user_va(pid, va, bytes);
        return Err(MmioMapError::NoVaSpace);
    }
    let phys = PhysAddr::new(w.page_start);
    let size = w.page_bytes as usize;
    let wc = kind == MemType::WriteCombining;
    if crate::memory::paging::map_user_mmio(user_va, phys, size, wc).is_err() {
        let _ = crate::memory::paging::unmap_user_mmio(user_va, size);
        grant::release_user_va(pid, va, bytes);
        return Err(MmioMapError::MapFailed);
    }
    Ok(user_va)
}
