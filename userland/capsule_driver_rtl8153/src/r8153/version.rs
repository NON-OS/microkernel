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

//! The chip version, from the bits Linux __rtl_get_hw_ver reads: the dword
//! at PLA_TCR0, shifted down 16 and masked with VERSION_MASK (0x7cf0).
//! RTL_VER_03 to 06 are the RTL8153 (r8153_init), 08 and 09 the RTL8153B
//! (r8153b_init); every other version is refused by name.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Version {
    V03,
    V04,
    V05,
    V06,
    V08,
    V09,
}

const VERSION_MASK: u32 = 0x7cf0;

pub fn version(tcr0: u32) -> Result<Version, &'static str> {
    match (tcr0 >> 16) & VERSION_MASK {
        0x5c00 => Ok(Version::V03),
        0x5c10 => Ok(Version::V04),
        0x5c20 => Ok(Version::V05),
        0x5c30 => Ok(Version::V06),
        0x6000 => Ok(Version::V08),
        0x6010 => Ok(Version::V09),
        // RTL_VER_01, 02 and 07: r8152b_init, 100 Mb/s.
        0x4c00 | 0x4c10 | 0x4800 => Err("an RTL8152, not an RTL8153"),
        // RTL_TEST_01, RTL_VER_10 to 13 and 15: r8156_init, r8156b_init.
        0x7010 | 0x7020 | 0x7030 | 0x7400 | 0x7410 | 0x7420 => Err("an RTL8156, not an RTL8153"),
        // RTL_VER_14: r8153c_init.
        0x6400 => Err("an RTL8153C, whose init this driver lacks"),
        _ => Err("unknown chip version"),
    }
}

impl Version {
    /// RTL_VER_08 and 09, which r8152.c drives through its r8153b paths.
    pub fn is_8153b(self) -> bool {
        matches!(self, Version::V08 | Version::V09)
    }

    pub fn name(self) -> &'static [u8] {
        match self {
            Version::V03 => b"RTL8153 (RTL_VER_03)",
            Version::V04 => b"RTL8153 (RTL_VER_04)",
            Version::V05 => b"RTL8153 (RTL_VER_05)",
            Version::V06 => b"RTL8153 (RTL_VER_06)",
            Version::V08 => b"RTL8153B (RTL_VER_08)",
            Version::V09 => b"RTL8153B (RTL_VER_09)",
        }
    }
}
