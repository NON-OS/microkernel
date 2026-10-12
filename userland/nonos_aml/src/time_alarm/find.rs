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

//! The present Time and Alarm device, by _HID or _CID; the first one is
//! the platform's, there being one clock.

use crate::namespace::Id;
use crate::platform::{DeviceInfo, PRESENT};

pub const TIME_ALARM_HID: &[u8] = b"ACPI000E";

pub fn time_alarm_device(devices: &[DeviceInfo]) -> Option<Id> {
    let ours = |d: &&DeviceInfo| {
        let mut ids = d.hid.iter().chain(&d.cids);
        d.sta & PRESENT != 0 && ids.any(|h| h.bytes() == TIME_ALARM_HID)
    };
    devices.iter().find(ours).map(|d| d.id)
}
