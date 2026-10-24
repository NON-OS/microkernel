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

//! DPCD addresses this driver reads or writes (DisplayPort 1.4a section
//! 2.9.3, the DPCD field map), named as the standard names them.

pub const DPCD_REV: u32 = 0x000;
pub const MAX_LINK_RATE: u32 = 0x001;
/// Bits 4:0 lanes; bit 6 TPS3 supported; bit 7 enhanced framing.
pub const MAX_LANE_COUNT: u32 = 0x002;
/// Bit 7: TPS4 supported (1.4).
pub const MAX_DOWNSPREAD: u32 = 0x003;
/// Bits 6:0 the channel equalisation read interval; bit 7 says an
/// extended receiver capability block sits at 0x2200.
pub const TRAINING_AUX_RD_INTERVAL: u32 = 0x00E;

pub const LINK_BW_SET: u32 = 0x100;
/// Bit 7 enables enhanced framing.
pub const LANE_COUNT_SET: u32 = 0x101;
/// Bits 1:0 the pattern (1 TPS1, 2 TPS2, 3 TPS3, 0 off); bit 5 disables
/// scrambling, as the first two patterns need.
pub const TRAINING_PATTERN_SET: u32 = 0x102;
/// TRAINING_LANE0_SET; lanes 1 to 3 follow at 0x104-0x106.
pub const TRAINING_LANE0_SET: u32 = 0x103;
pub const MAIN_LINK_CHANNEL_CODING_SET: u32 = 0x108;

/// LANE0_1_STATUS; LANE2_3_STATUS, LANE_ALIGN_STATUS_UPDATED,
/// SINK_STATUS and ADJUST_REQUEST_LANE0_1/2_3 follow to 0x207.
pub const LANE0_1_STATUS: u32 = 0x202;
pub const SET_POWER: u32 = 0x600;

/// Per-lane status bits, in each lane's nibble.
pub const CR_DONE: u8 = 1 << 0;
pub const CHANNEL_EQ_DONE: u8 = 1 << 1;
pub const SYMBOL_LOCKED: u8 = 1 << 2;
/// LANE_ALIGN_STATUS_UPDATED bit 0.
pub const INTERLANE_ALIGN_DONE: u8 = 1 << 0;

/// MAX_LINK_RATE and LINK_BW_SET codes: the lane rate in 0.27 Gbps.
pub fn rate_code(mbps: u32) -> Option<u8> {
    match mbps {
        1620 => Some(0x06),
        2700 => Some(0x0A),
        5400 => Some(0x14),
        8100 => Some(0x1E),
        _ => None,
    }
}

pub fn rate_mbps(code: u8) -> Option<u32> {
    match code {
        0x06 => Some(1620),
        0x0A => Some(2700),
        0x14 => Some(5400),
        0x1E => Some(8100),
        _ => None,
    }
}
