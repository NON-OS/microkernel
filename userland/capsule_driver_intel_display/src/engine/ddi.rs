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

//! A DDI's buffer and its DisplayPort transport (SKL PRM Vol 2c,
//! DDI_BUF_CTL, DP_TP_CTL, DDI_BUF_TRANS). The buffer's drive comes from
//! one of ten translation entries, which firmware fills with the values
//! for its board; this driver picks an entry and never writes them.

use crate::dp::lane::Drive;

pub fn buf_ctl(ddi: u32) -> u32 {
    0x6_4000 + 0x100 * ddi
}
pub fn tp_ctl(ddi: u32) -> u32 {
    0x6_4040 + 0x100 * ddi
}
/// The low dword of translation entry `e` of DDI `ddi`.
pub fn buf_trans(ddi: u32, e: u32) -> u32 {
    0x6_4E00 + 0x60 * ddi + 8 * e
}

/// DDI_BUF_CTL: enable, translation select (27:24), idle status (7),
/// DisplayPort port width (3:1: 000 one lane, 001 two, 011 four).
pub const BUF_ENABLE: u32 = 1 << 31;
pub const TRANS_SHIFT: u32 = 24;
pub const TRANS_MASK: u32 = 0xF << 24;
pub const BUF_IDLE: u32 = 1 << 7;
pub const WIDTH_MASK: u32 = 0x7 << 1;

pub fn width_bits(lanes: u32) -> u32 {
    (lanes.saturating_sub(1).min(3)) << 1
}

/// DP_TP_CTL: enable, enhanced framing (18), pattern (10:8: 000 TPS1,
/// 001 TPS2, 010 idle, 011 normal, 100 TPS3).
pub const TP_ENABLE: u32 = 1 << 31;
pub const TP_ENHANCED: u32 = 1 << 18;
pub const PATTERN_MASK: u32 = 0x7 << 8;
pub const TPS1: u32 = 0 << 8;
pub const TPS2: u32 = 1 << 8;
pub const IDLE: u32 = 2 << 8;
pub const NORMAL: u32 = 3 << 8;
pub const TPS3: u32 = 4 << 8;

/// The translation entry for a drive level: the ten legal swing and
/// pre-emphasis pairs in the standard's order, swing 0 with pre-emphasis
/// 0 to 3 first, through swing 3 with none.
pub fn entry(d: Drive) -> u32 {
    let d = d.legal();
    let before = [0, 4, 7, 9][d.swing as usize];
    before + d.pre as u32
}
