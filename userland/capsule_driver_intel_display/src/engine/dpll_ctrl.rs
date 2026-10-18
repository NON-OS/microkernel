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

//! The fields of DPLL_CTRL1, DPLL_CTRL2 and DPLL_STATUS (SKL PRM Vol 2c);
//! the bit layouts are unverified against the volume.

/// DPLL_CTRL1, six bits per DPLL: override (0), link rate (3:1), SSC (4),
/// HDMI mode (5). The link rate code for a DisplayPort lane rate: the
/// DDI clock is half the bit rate (unverified encoding).
pub fn link_rate_code(mbps: u32) -> Option<u32> {
    match mbps {
        5400 => Some(0b000),
        2700 => Some(0b001),
        1620 => Some(0b010),
        3240 => Some(0b011),
        2160 => Some(0b100),
        4320 => Some(0b101),
        _ => None,
    }
}

/// The DPLL_CTRL1 bits for `dpll`: DisplayPort at `mbps`, or HDMI.
pub fn ctrl1_bits(dpll: u32, mode: Option<u32>) -> Option<(u32, u32)> {
    let shift = dpll * 6;
    let bits = match mode {
        Some(mbps) => 1 | (link_rate_code(mbps)? << 1),
        None => 1 | (1 << 5),
    };
    Some((0x3F << shift, bits << shift))
}

/// DPLL_CTRL2 for DDI `ddi` (A = 0): its clock select (2:1 of its
/// three bits) and override (0), and its clock-off bit at 15 + ddi.
pub fn ctrl2_bits(ddi: u32, dpll: u32) -> (u32, u32) {
    let mask = (7 << (ddi * 3)) | (1 << (15 + ddi));
    (mask, (1 | (dpll << 1)) << (ddi * 3))
}

/// DPLL_STATUS lock bit of `dpll`: bit 0 of its byte.
pub fn locked(status: u32, dpll: u32) -> bool {
    status & (1 << (dpll * 8)) != 0
}
