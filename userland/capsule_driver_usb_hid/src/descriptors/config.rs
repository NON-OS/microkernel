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

//! What a configuration descriptor says about the device as a whole: how
//! long it is, which value selects it, and whether the device is a hub.

use super::types::{DT_CONFIGURATION, DT_INTERFACE};

pub const CLASS_HUB: u8 = 0x09;
/// The configuration descriptor's own header, which carries wTotalLength.
pub const CONFIG_HEADER_LEN: u16 = 9;

/// wTotalLength from the header, when `raw` starts with one.
pub fn total_length(raw: &[u8]) -> Option<u16> {
    if raw.len() < CONFIG_HEADER_LEN as usize || raw[1] != DT_CONFIGURATION {
        return None;
    }
    Some(u16::from_le_bytes([raw[2], raw[3]]))
}

/// bConfigurationValue, the value SET_CONFIGURATION selects this
/// configuration with. Most devices use 1; nothing requires it.
pub fn configuration_value(raw: &[u8]) -> Option<u8> {
    total_length(raw).map(|_| raw[5])
}

/// Whether any interface in `raw` is of the hub class: the device is
/// brought up as a hub, never bound as HID.
pub fn is_hub(raw: &[u8]) -> bool {
    let end = total_length(raw).map_or(0, |t| (t as usize).min(raw.len()));
    let mut i = CONFIG_HEADER_LEN as usize;
    while i + 2 <= end {
        let len = raw[i] as usize;
        if len < 2 || i + len > end {
            return false;
        }
        if raw[i + 1] == DT_INTERFACE && len >= 9 && raw[i + 5] == CLASS_HUB {
            return true;
        }
        i += len;
    }
    false
}
