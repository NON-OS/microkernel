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

//! The two questions any caller may put to the IOMMU layer.

use super::backend;
use super::capabilities::IommuCapabilities;
use super::vendor::IommuVendor;

/*
 * Decided on the first call and fixed after it, so the first call has to
 * come after ACPI parsing. The boot makes it from init_dma_protection.
 */
pub fn select_vendor() -> IommuVendor {
    backend::select_vendor()
}

/// The guarantees in force at the moment of the call. Never triggers selection.
pub fn capabilities() -> IommuCapabilities {
    backend::capabilities()
}
