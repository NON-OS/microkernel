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
//! The combo PHY's registers, one block per port. Each block sits at its
//! port's base (A 162000h, B 6C000h, C 160000h). Lane registers repeat for
//! the AUX lane (+300h), the group (+600h) and lanes 0 to 3 (+800h, +100h
//! each), as the instance lists of Vol 2c pp. 885 to 953 give them. A write
//! to the group reaches every lane; a read of the group is not reliable
//! (Vol 12 p. 391), so a read goes to lane 0.

use super::clka::Combo;

/// The port's block base.
pub fn base(p: Combo) -> u32 {
    [0x16_2000, 0x6_C000, 0x16_0000][p as usize]
}

/// PORT_CL_DW5 (+014h, Vol 2c p. 885) and PORT_CL_DW10 (+028h, p. 887).
pub const CL_DW5: u32 = 0x014;
pub const CL_DW10: u32 = 0x028;
/// PORT_COMP_DW0, DW1, DW3, DW8, DW9 and DW10 (+100h to +128h, pp. 896 to
/// 903).
pub const COMP_DW0: u32 = 0x100;
pub const COMP_DW1: u32 = 0x104;
pub const COMP_DW3: u32 = 0x10C;
pub const COMP_DW8: u32 = 0x120;
pub const COMP_DW9: u32 = 0x124;
pub const COMP_DW10: u32 = 0x128;
/// Lane registers, as offsets inside a lane block: PORT_PCS_DW1 (+004h,
/// p. 904), PORT_TX_DW2 (+088h, p. 935), DW4 (+090h, p. 938), DW5 (+094h,
/// p. 941), DW7 (+09Ch, p. 948) and DW8 (+0A0h, p. 951).
pub const PCS_DW1: u32 = 0x004;
pub const TX_DW2: u32 = 0x088;
pub const TX_DW4: u32 = 0x090;
pub const TX_DW5: u32 = 0x094;
pub const TX_DW7: u32 = 0x09C;
pub const TX_DW8: u32 = 0x0A0;

/// A port-wide register of `p`.
pub fn port(p: Combo, reg: u32) -> u32 {
    base(p) + reg
}

/// The group instance of lane register `reg`.
pub fn group(p: Combo, reg: u32) -> u32 {
    base(p) + 0x600 + reg
}

/// Lane `n`'s instance (0 to 3) of lane register `reg`.
pub fn lane(p: Combo, n: u32, reg: u32) -> u32 {
    base(p) + 0x800 + 0x100 * (n & 3) + reg
}
