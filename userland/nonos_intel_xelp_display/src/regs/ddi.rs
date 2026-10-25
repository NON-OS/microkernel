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
//! The DDI's own registers for a combo port: its buffer control, its PHY
//! misc register and its IO power in the driver's power well control.

use super::clka::Combo;

/// DDI_BUF_CTL_A, B, C: 64000h, 64100h, 64200h (Vol 2c p. 352).
pub fn buf_ctl(p: Combo) -> u32 {
    0x6_4000 + 0x100 * p as u32
}
/// DDI_BUF_CTL bit 31, DDI Buffer Enable (p. 353).
pub const BUF_ENABLE: u32 = 1 << 31;
/// DDI_BUF_CTL bit 7, DDI Idle Status; 1b is idle (p. 354).
pub const BUF_IDLE: u32 = 1 << 7;
/// DDI_BUF_CTL 3:1, DP Port Width: 000b x1, 001b x2, 011b x4 (p. 355).
pub const BUF_WIDTH: u32 = 7 << 1;

/// PHY_MISC_A, B, C: 64C00h, 64C04h, 64C08h (p. 664).
pub fn phy_misc(p: Combo) -> u32 {
    0x6_4C00 + 4 * p as u32
}
/// PHY_MISC bit 23, DE to IO Comp Pwr Down (p. 664).
pub const MISC_COMP_PWR_DOWN: u32 = 1 << 23;

/// PWR_WELL_CTL_DDI2, the driver's copy, 45454h (pp. 1071, 1072).
pub const PWR_WELL_CTL_DDI2: u32 = 0x4_5454;
/// A port's IO power request and state bits: A 1 and 0, B 3 and 2, C 5
/// and 4 (pp. 1074, 1075).
pub fn io_power(p: Combo) -> (u32, u32) {
    let n = 2 * p as u32;
    (1 << (n + 1), 1 << n)
}
