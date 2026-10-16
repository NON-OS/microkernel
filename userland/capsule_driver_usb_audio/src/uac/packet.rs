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

//! What an isochronous endpoint carries a service interval, from its
//! wMaxPacketSize (USB 2.0 9.6.6, as recalled, unverified): bits 10:0 the
//! bytes a transaction, bits 12:11 the transactions added a microframe at
//! high speed (3 reserved), bits 15:13 reserved. An isochronous transaction
//! carries at most 1023 bytes at full speed and 1024 faster. A value
//! outside those bounds is the device's error, refused, never trimmed.

use super::timing::SPEED_HIGH;

/// Bytes the endpoint moves a service interval at port speed `speed`.
pub fn per_interval(speed: u8, max_packet: u16) -> Option<u32> {
    let size = u32::from(max_packet & 0x7FF);
    let extra = u32::from((max_packet >> 11) & 0x3);
    let most = if speed >= SPEED_HIGH { 1024 } else { 1023 };
    let extra_allowed = speed == SPEED_HIGH && extra < 3;
    if max_packet >> 13 != 0 || size > most || (extra != 0 && !extra_allowed) {
        return None;
    }
    Some(size * (extra + 1))
}
