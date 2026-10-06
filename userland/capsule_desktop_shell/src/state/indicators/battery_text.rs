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

//! The menu bar's battery label, apart from the syscall so the rule is
//! proven on the host (desktop_proofs).

/// `MkBatteryStatus` errno for "the firmware declares no battery".
pub const STATUS_NO_BATTERY: i64 = -19;

/// The longest label, "Battery status unavailable".
pub const LABEL_MAX: usize = 26;

pub const NO_BATTERY: &[u8] = b"No battery";
pub const UNAVAILABLE: &[u8] = b"Battery status unavailable";

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

/// "57%" for a reading, "No battery" or "Battery status unavailable"
/// otherwise: never a number the kernel did not give.
pub fn label(b: Battery, buf: &mut [u8; LABEL_MAX]) -> usize {
    let p = match b {
        Battery::None => {
            buf[..NO_BATTERY.len()].copy_from_slice(NO_BATTERY);
            return NO_BATTERY.len();
        }
        Battery::Unavailable => {
            buf[..UNAVAILABLE.len()].copy_from_slice(UNAVAILABLE);
            return UNAVAILABLE.len();
        }
        Battery::Percent(p) => p.min(100),
    };
    let mut n = 0;
    if p >= 100 {
        buf[0] = b'1';
        buf[1] = b'0';
        buf[2] = b'0';
        n = 3;
    } else {
        if p >= 10 {
            buf[n] = b'0' + (p / 10) as u8;
            n += 1;
        }
        buf[n] = b'0' + (p % 10) as u8;
        n += 1;
    }
    buf[n] = b'%';
    n + 1
}
