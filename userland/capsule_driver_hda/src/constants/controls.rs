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

//! Payload bits of the power state, pin control, EAPD, pin sense and amp
//! gain/mute verbs, by the Intel High Definition Audio Specification 1.0a,
//! section 7.3.3.

/// GET_POWER_STATE answers the state asked for in 3:0, the state reached in
/// 7:4, and in bit 8 that the widget could not reach it.
pub const POWER_ACTUAL_SHIFT: u32 = 4;
pub const POWER_ERROR: u32 = 1 << 8;
pub const POWER_D0: u8 = 0x00;

pub const PIN_OUT_ENABLE: u8 = 0x40;
pub const PIN_HP_ENABLE: u8 = 0x80;
pub const EAPD_ENABLE: u8 = 0x02;
pub const PIN_SENSE_PRESENT: u32 = 1 << 31;

/// SET_AMP_GAIN_MUTE payload: which amp in 15:12, input index in 11:8,
/// mute in bit 7, gain in 6:0.
pub const AMP_SET_OUTPUT: u16 = 1 << 15;
pub const AMP_SET_INPUT: u16 = 1 << 14;
pub const AMP_SET_LEFT: u16 = 1 << 13;
pub const AMP_SET_RIGHT: u16 = 1 << 12;
pub const AMP_MUTE: u16 = 1 << 7;
pub const AMP_OUT_UNMUTE: u16 = AMP_SET_OUTPUT | AMP_SET_LEFT | AMP_SET_RIGHT;
pub const AMP_IN_UNMUTE: u16 = AMP_SET_INPUT | AMP_SET_LEFT | AMP_SET_RIGHT;
