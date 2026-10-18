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

use crate::kernel_core::surface_registry::types::RegistryError;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::api::map_page_in_asid;
use crate::memory::paging::types::PagePermissions;

/* Map `frames` at consecutive pages from `base` in `asid`, user read-only
 * unless the receiver was named to write (K20). */
pub(super) fn map_frames(
    asid: u32,
    base: VirtAddr,
    frames: &[PhysAddr],
    write: bool,
) -> Result<(), RegistryError> {
    let perms = if write { PagePermissions::user_rw() } else { PagePermissions::user_ro() };
    for (i, frame) in frames.iter().enumerate() {
        let va = VirtAddr::new(base.as_u64() + (i as u64) * 4096);
        map_page_in_asid(asid, va, *frame, perms).map_err(|_| RegistryError::MapFailed)?;
    }
    Ok(())
}
