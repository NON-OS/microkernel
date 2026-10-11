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

//! What the model host reports and how it misbehaves.

#[derive(Debug, Clone)]
pub struct HostCfg {
    pub caps: u32,
    pub caps1: u32,
    pub version: u16,
    /// Writes of SD Bus Power on that do not stick.
    pub power_ignores: u32,
    /// Reads of Software Reset that still show the bits set.
    pub reset_reads: u32,
    /// Card detect reads present.
    pub cd_inserted: bool,
    /// The card's write-protect switch is on.
    pub write_protected: bool,
    /// Once this many commands are issued, the card is taken out and an
    /// identical one put in before the last is answered: a swap mid-request.
    pub swap_at_cmd: Option<usize>,
}

/// Intel Gemini Lake eMMC-like: spec 3.00, 200 MHz, ADMA2, 64-bit,
/// high speed, 1.8 V only, embedded slot, 8-bit not advertised.
pub const GLK_CAPS: u32 = (200 << 8) | 1 << 19 | 1 << 21 | 1 << 26 | 1 << 28 | 1 << 30;

impl Default for HostCfg {
    fn default() -> Self {
        Self {
            caps: GLK_CAPS,
            caps1: 0x0000_0807,
            version: 0x1002,
            power_ignores: 0,
            reset_reads: 2,
            cd_inserted: true,
            write_protected: false,
            swap_at_cmd: None,
        }
    }
}
