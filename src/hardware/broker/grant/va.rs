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

use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory::addr::VirtAddr;

// Snapshot for diagnostics. The lock is dropped before the
// caller sees the data.

// User MMIO virtual-address allocator. Each grant gets a fresh
// region inside `[USER_MMIO_BASE, USER_MMIO_END)`; the bumping
// counter is global because each capsule has its own page tables,
// so VA reuse across address spaces is harmless. Within one address
// space the bump never wraps for the life of the run; if it ever
// does, that is a real exhaustion and the request fails.
pub(in super::super) use super::super::windows::{USER_MMIO_BASE, USER_MMIO_END};
const PAGE_SIZE: u64 = 4096;

static NEXT_USER_MMIO_VA: AtomicU64 = AtomicU64::new(USER_MMIO_BASE);

// Reserve `pages * 4 KiB` of user VA in the MMIO grant region. A
// guard page is added between adjacent grants so a runaway access
// cannot silently spill into the next grant. Returns `None` on
// region exhaustion.
pub(in super::super) fn reserve_user_va(pages: u64) -> Option<VirtAddr> {
    let bytes = pages.checked_mul(PAGE_SIZE)?;
    let with_guard = bytes.checked_add(PAGE_SIZE)?;
    let base = NEXT_USER_MMIO_VA.fetch_add(with_guard, Ordering::SeqCst);
    let end = base.checked_add(bytes)?;
    if end > USER_MMIO_END {
        return None;
    }
    Some(VirtAddr::new(base))
}
