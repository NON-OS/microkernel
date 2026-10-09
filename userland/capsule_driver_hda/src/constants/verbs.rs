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

//! Codec verbs and parameters, by the Intel High Definition Audio
//! Specification 1.0a, section 7.3, with the names Linux gives them in
//! include/sound/hda_verbs.h.

pub const VERB_GET_CONNECT_LIST: u16 = 0xf02;
pub const VERB_GET_POWER_STATE: u16 = 0xf05;
pub const VERB_GET_PIN_SENSE: u16 = 0xf09;
pub const VERB_GET_CONFIG_DEFAULT: u16 = 0xf1c;
pub const VERB_GET_SUBSYSTEM_ID: u16 = 0xf20;
pub const VERB_GET_PROC_COEF: u16 = 0xc00;

pub const VERB_SET_CONNECT_SEL: u16 = 0x701;
pub const VERB_SET_POWER_STATE: u16 = 0x705;
pub const VERB_SET_CHANNEL_STREAMID: u16 = 0x706;
pub const VERB_SET_PIN_WIDGET_CONTROL: u16 = 0x707;
pub const VERB_SET_PIN_SENSE: u16 = 0x709;
pub const VERB_SET_BEEP_CONTROL: u16 = 0x70a;
pub const VERB_SET_EAPD_BTLENABLE: u16 = 0x70c;
pub const VERB_SET_GPIO_DATA: u16 = 0x715;
pub const VERB_SET_GPIO_MASK: u16 = 0x716;
pub const VERB_SET_GPIO_DIRECTION: u16 = 0x717;

/// The four-bit verbs, whose payload is sixteen bits wide.
pub const VERB_SET_STREAM_FORMAT: u8 = 0x2;
pub const VERB_SET_AMP_GAIN_MUTE: u8 = 0x3;
pub const VERB_SET_PROC_COEF: u8 = 0x4;
pub const VERB_SET_COEF_INDEX: u8 = 0x5;

pub const PARAM_SUBNODE_COUNT: u16 = 0x04;
pub const PARAM_FUNCTION_GROUP_TYPE: u16 = 0x05;
pub const PARAM_AUDIO_WIDGET_CAP: u16 = 0x09;
pub const PARAM_PCM: u16 = 0x0a;
pub const PARAM_PIN_CAP: u16 = 0x0c;
pub const PARAM_AMP_IN_CAP: u16 = 0x0d;
pub const PARAM_CONNLIST_LEN: u16 = 0x0e;
pub const PARAM_GPIO_CAP: u16 = 0x11;
pub const PARAM_AMP_OUT_CAP: u16 = 0x12;
