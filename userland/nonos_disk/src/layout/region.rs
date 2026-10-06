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

//! The four partitions, in the order they sit on the disk and in the table.

use super::extent::Extent;
use super::plan::Layout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    Store,
    Plan,
    Data,
    Esp,
}

impl Region {
    pub const ALL: [Region; 4] = [Region::Store, Region::Plan, Region::Data, Region::Esp];

    /// What it holds, in the words the screens use.
    pub fn what(self) -> &'static str {
        match self {
            Region::Store => "the store",
            Region::Plan => "the disk plan",
            Region::Data => "the data volume",
            Region::Esp => "the boot partition",
        }
    }
}

impl Layout {
    pub fn extent(&self, region: Region) -> Extent {
        match region {
            Region::Store => self.store,
            Region::Plan => self.plan,
            Region::Data => self.data,
            Region::Esp => self.esp,
        }
    }
}
