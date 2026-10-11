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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PolicyVerdict {
    Valid = 0,
    /// Not a Linux listing: no guest policy is asked for.
    NotLinux = 1,
    /// A vendor program shipped in the image: its policy is the vendor
    /// table's, so any policy carried here must name no listing or release.
    Vendor = 2,
    Missing = 3,
    /// Not 256 bytes, or not "NLXPOL01" version 1.
    BadHeader = 4,
    /// Its listing and release ids are not this release's.
    WrongIds = 5,
}

impl PolicyVerdict {
    pub fn passes(self) -> bool {
        matches!(self, PolicyVerdict::Valid | PolicyVerdict::NotLinux | PolicyVerdict::Vendor)
    }
}
