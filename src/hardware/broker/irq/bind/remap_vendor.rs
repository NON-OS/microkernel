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
//! Which interrupt remapping a broker message goes through: AMD's when its
//! units are in service, else VT-d's when built, else none.

use super::message::Routed;
use super::remap_rule::Attempt;
use crate::drivers::pci::types::PciAddress;

pub(super) fn attempt(address: &PciAddress, vector: u8, dest: u8) -> (Attempt, Option<Routed>) {
    #[cfg(feature = "nonos-iommu-amdvi")]
    if crate::arch::x86_64::iommu::amd_vi::intr::is_remapping() {
        return super::amd_remap::attempt(address, vector, dest);
    }
    #[cfg(feature = "nonos-iommu-intremap")]
    return super::remap::attempt(address, vector, dest);
    #[cfg(not(feature = "nonos-iommu-intremap"))]
    {
        let _ = (address, vector, dest);
        (Attempt::Off, None)
    }
}
