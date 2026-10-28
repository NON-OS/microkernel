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

/// The report word's bits (`power_report.rs`, MkBatteryStatus form 1).
const PRESENT: i64 = 1 << 8;
const CHARGING: i64 = 1 << 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Battery {
    /// A real charge reading, 0..=100.
    Percent(u32),
    /// A real charge reading, and firmware says it is charging.
    Charging(u32),
    /// The machine has no battery.
    None,
    /// There is (or may be) a battery but its charge cannot be read.
    Unavailable,
}

impl Battery {
    /// Decode the state form's answer: an errno as `from_status` reads it,
    /// else the report's word, its charge believed only up to 100.
    pub fn from_state(rc: i64) -> Self {
        match (rc, rc & 0xFF) {
            (..=-1, _) => Self::from_status(rc),
            (w, _) if w & PRESENT == 0 => Self::None,
            (_, p) if p > 100 => Self::Unavailable,
            (w, p) if w & CHARGING != 0 => Self::Charging(p as u32),
            (_, p) => Self::Percent(p as u32),
        }
    }

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
            Self::Percent(p) | Self::Charging(p) => Some(p),
            _ => None,
        }
    }
}
