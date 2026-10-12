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

//! What the predefined \_OS, \_OSI and \_REV answer.

/// The answers are data, so the capsule can be built to answer as the owner
/// decides. `DEFAULT` answers the Windows version strings in `osi::WINDOWS`
/// and nothing else; \_OS is "Microsoft Windows NT", the name firmware written
/// for the NT family compares \_OS against (section 5.7.3 leaves the string
/// to the operating system); \_REV is 2, what Windows answers, since firmware
/// has been seen to take a higher \_REV for a Linux it treats differently
/// (unverified beyond published reports).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Personality {
    pub os: &'static [u8],
    pub osi: &'static [&'static [u8]],
    pub rev: u64,
}

impl Personality {
    pub const DEFAULT: Personality =
        Personality { os: b"Microsoft Windows NT", osi: super::osi::WINDOWS, rev: 2 };

    /// The default, answering the Windows strings only up to and including
    /// `newest`: for a machine whose firmware misbehaves with the newer paths.
    pub fn windows_up_to(newest: &[u8]) -> Personality {
        let n = super::osi::WINDOWS.iter().position(|s| *s == newest).map_or(0, |i| i + 1);
        Personality { osi: &super::osi::WINDOWS[..n], ..Self::DEFAULT }
    }

    /// \_OSI(string): whether the string is one this build answers.
    pub fn answers(&self, s: &[u8]) -> bool {
        self.osi.contains(&s)
            || super::osi::FEATURES.contains(&s)
            || super::osi::VENDORS.contains(&s)
    }
}

impl Default for Personality {
    fn default() -> Self {
        Self::DEFAULT
    }
}
