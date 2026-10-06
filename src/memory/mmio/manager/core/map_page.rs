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

use super::super::super::constants::{
    VM_FLAG_CACHE_DISABLE, VM_FLAG_NX, VM_FLAG_USER, VM_FLAG_WRITABLE, VM_FLAG_WRITE_COMBINE,
};
use super::super::super::error::{MmioError, MmioResult};
use super::types::MmioManager;
use crate::memory::addr::{PhysAddr, VirtAddr};

impl MmioManager {
    pub(super) fn map_page(&self, va: VirtAddr, pa: PhysAddr, vm_flags: u32) -> MmioResult<()> {
        use crate::memory::paging::manager;
        use crate::memory::paging::types::PagePermissions;
        let mut perms = PagePermissions::READ;
        if (vm_flags & VM_FLAG_WRITABLE) != 0 {
            perms = perms | PagePermissions::WRITE;
        }
        if (vm_flags & VM_FLAG_USER) != 0 {
            perms = perms | PagePermissions::USER;
        }
        if (vm_flags & VM_FLAG_NX) == 0 {
            perms = perms | PagePermissions::EXECUTE;
        }
        /*
         * The framebuffer is written write-combining through PAT entry 1
         * (PWT alone) once every CPU's table has it, as Linux maps efifb
         * and simpledrm. Before, it had PCD alone, UC-, which under an
         * uncached MTRR, or a write-back default with no MTRR over the
         * aperture, comes out uncached (SDM Vol. 3A table 12-7): every
         * 4-byte store of a present was its own bus write. PAT WC is WC
         * whatever the MTRR says. With no PAT the old UC- stays.
         */
        if (vm_flags & VM_FLAG_WRITE_COMBINE) != 0 {
            perms = perms | write_combining();
            return manager::map_page(va, pa, perms).map_err(|_| MmioError::MappingFailed);
        }
        if (vm_flags & VM_FLAG_CACHE_DISABLE) != 0 {
            perms = perms | PagePermissions::NO_CACHE | PagePermissions::DEVICE;
        }
        manager::map_page(va, pa, perms).map_err(|_| MmioError::MappingFailed)
    }

    pub(super) fn unmap_page(&self, va: VirtAddr) -> MmioResult<()> {
        crate::memory::paging::manager::unmap_page(va)
            .map(|_| ())
            .map_err(|_| MmioError::UnmapFailed)
    }
}

fn write_combining() -> crate::memory::paging::types::PagePermissions {
    use crate::memory::paging::types::PagePermissions;
    #[cfg(target_arch = "x86_64")]
    if crate::arch::x86_64::pat::wc_ready() {
        return PagePermissions::WRITE_THROUGH;
    }
    PagePermissions::NO_CACHE
}
