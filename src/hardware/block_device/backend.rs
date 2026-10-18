// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use core::fmt;

use super::disk::Disk;

/// One disk the block layer may ask: its row, and which of the row's disks.
#[derive(Clone, Copy)]
pub struct Backend {
    pub(super) disk: &'static Disk,
    pub(super) key: u32,
}

impl Backend {
    pub(super) const fn of(disk: &'static Disk, key: u32) -> Self {
        Self { disk, key }
    }

    /// What the probe log calls this disk.
    pub fn shown(&self) -> &'static str {
        (self.disk.shown)(self.key)
    }
}

impl PartialEq for Backend {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.disk, other.disk) && self.key == other.key
    }
}

impl Eq for Backend {}

impl fmt::Debug for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.shown())
    }
}
