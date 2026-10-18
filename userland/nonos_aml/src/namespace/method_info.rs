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

//! A method's flags.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MethodInfo {
    /// MethodFlags (section 20.2.5.2): ArgCount in bits 0-2, SerializeFlag in
    /// bit 3, SyncLevel in bits 4-7.
    pub flags: u8,
}

impl MethodInfo {
    pub fn args(&self) -> u8 {
        self.flags & 7
    }
    pub fn serialized(&self) -> bool {
        self.flags & 8 != 0
    }
    pub fn sync_level(&self) -> u8 {
        self.flags >> 4
    }
}
