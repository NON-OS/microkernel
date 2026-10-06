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

//! The device descriptor's fields a driver binds on (USB 2.0, table 9-8),
//! and a configuration descriptor's header (table 9-10).

use super::kinds::{CONFIG, DEVICE};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DeviceInfo {
    pub vendor: u16,
    pub product: u16,
    pub class: u8,
    pub configs: u8,
}

/// The device descriptor in `raw`, when it is one.
pub fn device_info(raw: &[u8]) -> Option<DeviceInfo> {
    if raw.len() < 18 || raw[0] < 18 || raw[1] != DEVICE {
        return None;
    }
    let vendor = u16::from_le_bytes([raw[8], raw[9]]);
    let product = u16::from_le_bytes([raw[10], raw[11]]);
    Some(DeviceInfo { vendor, product, class: raw[4], configs: raw[17] })
}

/// A configuration descriptor's wTotalLength and bConfigurationValue.
pub fn config_header(raw: &[u8]) -> Option<(u16, u8)> {
    if raw.len() < 9 || raw[0] < 9 || raw[1] != CONFIG {
        return None;
    }
    Some((u16::from_le_bytes([raw[2], raw[3]]), raw[5]))
}
