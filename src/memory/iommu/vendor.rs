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

//! Which remapping hardware the firmware described.

/*
 * Chosen from the ACPI tables, never from CPUID: QEMU presents an
 * intel-iommu under KVM on AMD hosts, and that machine is VT-d.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IommuVendor {
    /// No DMAR remapping unit and no IVRS table, or a build with no IOMMU backend.
    Absent,
    /// DMAR described at least one remapping unit.
    IntelVtd,
    /// IVRS is present and DMAR described no unit. This kernel does not drive AMD-Vi.
    AmdVi,
}

impl IommuVendor {
    /// The name the posture line prints.
    pub const fn name(self) -> &'static [u8] {
        match self {
            Self::Absent => b"none",
            Self::IntelVtd => b"intel-vt-d",
            Self::AmdVi => b"amd-vi",
        }
    }
}
