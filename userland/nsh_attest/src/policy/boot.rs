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

//! The boot applications a policy admits in PCR 4.

use alloc::vec::Vec;

use super::{Policy, PolicyError, MAX_BOOT_APPS};

impl Policy {
    /// The boot applications PCR 4 may hold besides the release's loader:
    /// a machine's boot manager, pinned when it was enrolled. None by
    /// default, so a policy admits the loader alone.
    pub fn with_boot_applications(mut self, apps: Vec<[u8; 32]>) -> Result<Self, PolicyError> {
        if apps.len() > MAX_BOOT_APPS || apps.iter().any(|a| a.iter().all(|b| *b == 0)) {
            return Err(PolicyError::BootApplications);
        }
        self.boot_apps = apps;
        Ok(self)
    }
}
