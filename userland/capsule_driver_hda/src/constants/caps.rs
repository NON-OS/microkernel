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

//! Parameter answers: function group type, audio widget capabilities, pin
//! and amp capabilities and supported PCM formats, by the Intel High
//! Definition Audio Specification 1.0a, section 7.3.4.

pub const FUNCTION_GROUP_AUDIO: u32 = 0x01;

pub const WIDGET_TYPE_DAC: u32 = 0x0;
pub const WIDGET_TYPE_MIXER: u32 = 0x2;
pub const WIDGET_TYPE_SELECTOR: u32 = 0x3;
pub const WIDGET_TYPE_PIN: u32 = 0x4;
pub const WIDGET_TYPE_BEEP: u32 = 0x7;

pub const WCAP_IN_AMP: u32 = 1 << 1;
pub const WCAP_OUT_AMP: u32 = 1 << 2;
pub const WCAP_AMP_OVRD: u32 = 1 << 3;
pub const WCAP_FORMAT_OVRD: u32 = 1 << 4;
pub const WCAP_CONN_LIST: u32 = 1 << 8;
pub const WCAP_DIGITAL: u32 = 1 << 9;
pub const WCAP_POWER: u32 = 1 << 10;

pub const PINCAP_TRIG_REQ: u32 = 1 << 1;
pub const PINCAP_PRES_DETECT: u32 = 1 << 2;
pub const PINCAP_HP_DRV: u32 = 1 << 3;
pub const PINCAP_OUT: u32 = 1 << 4;
pub const PINCAP_HDMI: u32 = 1 << 7;
pub const PINCAP_EAPD: u32 = 1 << 16;
pub const PINCAP_DP: u32 = 1 << 24;

pub const AMPCAP_OFFSET: u32 = 0x7f;

pub const PCM_RATE_48K: u32 = 1 << 6;
pub const PCM_BITS_16: u32 = 1 << 17;
