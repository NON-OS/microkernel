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

//! The permissions a user mapping asks for, and their leaf encoding.

use super::pte_flags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectionFlags {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
}

impl ProtectionFlags {
    pub fn new(read: bool, write: bool, execute: bool, user: bool) -> Self {
        Self { read, write, execute, user }
    }

    /// Writable and executable at once: what `AddressSpace::map_page` refuses.
    pub const fn is_wx_violation(&self) -> bool {
        self.write && self.execute
    }

    pub fn to_pte_flags(&self) -> u64 {
        let mut flags = pte_flags::PRESENT;

        if self.write {
            flags |= pte_flags::WRITABLE;
        }

        if self.user {
            flags |= pte_flags::USER_ACCESSIBLE;
        }

        if !self.execute {
            flags |= pte_flags::NO_EXECUTE;
        }

        flags
    }
}

impl Default for ProtectionFlags {
    fn default() -> Self {
        Self { read: true, write: false, execute: false, user: true }
    }
}
