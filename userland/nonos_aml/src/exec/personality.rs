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
/// decides. `DEFAULT` answers no \_OSI string at all, the safe choice until a
/// string's answer is justified (unit F 5 sets the table), names the
/// operating system as firmware written for ACPI expects to find it, and
/// gives \_REV as 2, the ACPI 2.0 revision firmware gates 64-bit integers on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Personality {
    pub os: &'static [u8],
    pub osi: &'static [&'static [u8]],
    pub rev: u64,
}

impl Personality {
    pub const DEFAULT: Personality = Personality { os: b"Microsoft Windows NT", osi: &[], rev: 2 };

    /// \_OSI(string): whether the string is one this build answers.
    pub fn answers(&self, s: &[u8]) -> bool {
        self.osi.contains(&s)
    }
}

impl Default for Personality {
    fn default() -> Self {
        Self::DEFAULT
    }
}
