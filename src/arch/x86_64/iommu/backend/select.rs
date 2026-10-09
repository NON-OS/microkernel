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

//! Which IOMMU this boot has, decided once from the firmware's tables.

use spin::Once;

use crate::arch::x86_64::acpi::has_table;
use crate::arch::x86_64::iommu::globals::is_present;
use crate::memory::iommu::IommuVendor;
use crate::sys::serial;

static SELECTED: Once<IommuVendor> = Once::new();

pub(crate) fn select_vendor() -> IommuVendor {
    *SELECTED.call_once(detect)
}

/// The vendor once selection has run, `None` before it.
pub(super) fn selected() -> Option<IommuVendor> {
    SELECTED.get().copied()
}

/*
 * Keyed on firmware tables, never on CPUID. `is_present` is set only once
 * DMAR yielded a remapping unit base, so a DMAR with no DRHD does not count.
 * VT-d wins when both tables exist because it is the one this kernel drives;
 * the AMD-Vi units are then named as unconfined rather than passed over.
 */
fn detect() -> IommuVendor {
    let ivrs = has_table(b"IVRS");
    if is_present() {
        if ivrs {
            serial::println(
                b"[IOMMU] IVRS also present; devices behind AMD-Vi units are not confined",
            );
        }
        return IommuVendor::IntelVtd;
    }
    if ivrs {
        #[cfg(not(feature = "nonos-iommu-amdvi"))]
        serial::println(
            b"[AMD-VI] IVRS present; no AMD-Vi backend in this kernel, IOMMU domains refused; DMA is unrestricted",
        );
        #[cfg(feature = "nonos-iommu-amdvi")]
        serial::println(b"[AMD-VI] IVRS present; the AMD-Vi units are brought up next");
        return IommuVendor::AmdVi;
    }
    serial::println(
        b"[IOMMU] no DMAR remapping unit and no IVRS table; IOMMU domains refused; DMA is unrestricted",
    );
    IommuVendor::Absent
}
