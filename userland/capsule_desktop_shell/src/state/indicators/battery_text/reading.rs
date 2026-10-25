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

//! What the battery syscall's answer says.

/// `MkBatteryStatus` errno for "the firmware declares no battery".
pub const STATUS_NO_BATTERY: i64 = -19;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Battery {
    /// A real charge reading, 0..=100.
    Percent(u32),
    /// The machine has no battery.
    None,
    /// There is (or may be) a battery but its charge cannot be read.
    Unavailable,
}

impl Battery {
    /// Decode the syscall's return: 0..=100 is a reading, `-ENODEV` is no
    /// battery, anything else is a battery the kernel cannot read.
    pub fn from_status(rc: i64) -> Self {
        if (0..=100).contains(&rc) {
            Self::Percent(rc as u32)
        } else if rc == STATUS_NO_BATTERY {
            Self::None
        } else {
            Self::Unavailable
        }
    }

    pub fn percent(self) -> Option<u32> {
        match self {
            Self::Percent(p) => Some(p),
            _ => None,
        }
    }
}
