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

// Mirrors the kernel `crate::memory::iommu` for the included broker source.
#[path = "../../../../../src/memory/iommu/error.rs"]
mod error;
pub use error::IommuError;
#[cfg(test)]
#[path = "../../../../../src/memory/iommu/dev_unconfined.rs"]
pub mod dev_unconfined;

/// The kernel's no-IOMMU posture is decided at boot; the host never takes it,
/// so the pure rules are proven by their inputs (confine_posture::open).
pub mod no_iommu {
    pub fn holds() -> bool {
        false
    }
}
