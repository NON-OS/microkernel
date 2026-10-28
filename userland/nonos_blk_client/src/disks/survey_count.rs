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

//! What a survey says as a whole: how many rows can be installed to, and
//! whether to say the firmware hides the disks or to look again.

use super::survey::{Disk, Survey};

impl Survey {
    /// The rows that can be installed to.
    pub fn targets(&self) -> usize {
        self.disks.iter().filter(|d| d.device.is_some()).count()
    }

    /// Whether to tell the person to change the firmware's storage mode: only
    /// with the RAID controller there and no disk, since a disk found means
    /// the firmware already lets this system reach one.
    pub fn raid_hides_disks(&self) -> bool {
        self.raid && self.targets() == 0
    }

    /// Whether a later look may find more: nothing to install to, or a driver missing.
    pub fn incomplete(&self) -> bool {
        self.targets() == 0 || self.disks.iter().any(Disk::missing)
    }
}
