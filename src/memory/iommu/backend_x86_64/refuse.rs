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

//! A domain call on a machine with no IOMMU this kernel drives, refused by name.

use crate::memory::iommu::IommuError;
use crate::sys::serial;

pub(super) fn amd_vi(op: &'static [u8]) -> IommuError {
    serial::print(b"[AMD-VI] refused ");
    serial::print(op);
    serial::println(b": no AMD-Vi backend in this kernel");
    IommuError::AmdViNotDriven
}

pub(super) fn absent(op: &'static [u8]) -> IommuError {
    serial::print(b"[IOMMU] refused ");
    serial::print(op);
    serial::println(b": no DMAR remapping unit and no IVRS table");
    IommuError::NoIommu
}
