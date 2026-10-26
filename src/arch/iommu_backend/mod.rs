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

//! The IOMMU backend for every build but x86_64 with its IOMMU feature,
//! mounted by `memory::iommu::backend` as its `inner` module. riscv64 has no
//! IOMMU backend, nor has x86_64 without that feature: there `absent` answers
//! that nothing translates, and the posture line says `none present`.

#[cfg(not(target_arch = "aarch64"))]
mod absent;
#[cfg(target_arch = "aarch64")]
mod smmu;

#[cfg(not(target_arch = "aarch64"))]
pub(in crate::memory::iommu) use absent::*;
#[cfg(target_arch = "aarch64")]
pub(in crate::memory::iommu) use smmu::*;
