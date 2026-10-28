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

//! The products of vendor 0x8087 Linux v6.12's btusb.c marks
//! BTUSB_INTEL_COMBINED, whose setup is `btintel_setup_combined`. Which
//! of them this loader can boot is decided by the hardware variant the
//! controller reports, not here: 0x07dc, 0x0a2a and 0x0aa7 are ROM parts
//! and 0x0a2b an SfP, and their variants are refused by name.

const COMBINED: [u16; 15] = [
    0x0025, 0x0026, 0x0029, 0x0032, 0x0033, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039, 0x07dc, 0x0a2a,
    0x0a2b, 0x0aa7, 0x0aaa,
];

pub fn combined(pid: u16) -> bool {
    COMBINED.contains(&pid)
}
