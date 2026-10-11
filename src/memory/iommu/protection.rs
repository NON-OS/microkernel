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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IommuProtection {
    pub read: bool,
    pub write: bool,
    /// The page is a device register, not memory: mapped with device
    /// attributes so the unit neither caches nor merges what goes through.
    pub mmio: bool,
}

impl IommuProtection {
    pub const READ: Self = Self { read: true, write: false, mmio: false };
    pub const WRITE: Self = Self { read: false, write: true, mmio: false };
    pub const READ_WRITE: Self = Self { read: true, write: true, mmio: false };
    /// An MSI doorbell: on Arm a device's message is a write the SMMU
    /// translates like any other, so the doorbell has to be mapped for it.
    pub const DOORBELL: Self = Self { read: false, write: true, mmio: true };

    pub const fn readable(&self) -> bool {
        self.read
    }

    pub const fn writable(&self) -> bool {
        self.write
    }
}

impl Default for IommuProtection {
    fn default() -> Self {
        Self::READ_WRITE
    }
}
