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

//! Which backend a domain call goes to, decided by the vendor selected at
//! boot.

use crate::memory::iommu::{IommuError, IommuVendor};

use super::{refuse, select};

/// The backend a call that passed routing goes to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Backend {
    Vtd,
    #[cfg(feature = "nonos-iommu-amdvi")]
    AmdVi,
}

/*
 * On VT-d, and before selection has run, the call goes straight to the VT-d
 * backend, which gates itself on translation being in service. On AMD-Vi it
 * goes to the AMD backend once every unit is in service. Otherwise the call
 * is refused by name instead of reaching tables no hardware walks.
 */
pub(super) fn route(op: &'static [u8]) -> Result<Backend, IommuError> {
    match select::selected() {
        None | Some(IommuVendor::IntelVtd) => Ok(Backend::Vtd),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Some(IommuVendor::AmdVi) if crate::arch::x86_64::iommu::amd_vi::is_enforcing() => {
            Ok(Backend::AmdVi)
        }
        Some(IommuVendor::AmdVi) => Err(refuse::amd_vi(op)),
        Some(IommuVendor::Absent) => Err(refuse::absent(op)),
    }
}
