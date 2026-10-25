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

//! The two headers a GPT disk carries (UEFI 2.10, 5.3.2, as recalled,
//! unverified). The primary sits at LBA 1, its array after it and wholly
//! before FirstUsableLBA. The backup sits at the disk's last LBA and names
//! LBA 1 as the other; its array lies wholly after LastUsableLBA and before
//! the backup header itself. The kernel only reads either one.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Which {
    Primary,
    Backup,
}

impl Which {
    /// The LBA this header must call its own on a disk of `sectors`.
    pub(crate) fn my_lba(self, sectors: u64) -> u64 {
        match self {
            Which::Primary => 1,
            Which::Backup => sectors.saturating_sub(1),
        }
    }

    /// Whether `alternate` is the other header's LBA as this one must name
    /// it: the backup names the primary at 1. The primary's is not held, as
    /// before: a disk grown after it was written still has its table.
    pub(crate) fn names_other(self, alternate: u64) -> bool {
        self == Which::Primary || alternate == 1
    }

    /// The sectors, `lo` up to `hi`, this header's array must lie in.
    pub(crate) fn array_room(self, first: u64, last: u64, sectors: u64) -> (u64, u64) {
        match self {
            Which::Primary => (2, first),
            Which::Backup => (last.saturating_add(1), sectors.saturating_sub(1)),
        }
    }
}
