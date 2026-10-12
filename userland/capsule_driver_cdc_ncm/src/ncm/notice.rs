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
//! The notification endpoint: where it is, and what NETWORK_CONNECTION
//! says (CDC 1.2 PSTN and ECM 1.2, 6.3.1, as recalled, unverified): an
//! 8-byte header shaped like a setup packet, bmRequestType A1h,
//! bNotification 00h, wValue 1 when the link is up and 0 when it is down.

use nonos_usbnet::desc::{walk, ENDPOINT, INTERFACE};
use nonos_usbnet::Notify;

const FROM_INTERFACE_CLASS: u8 = 0xA1;
const NETWORK_CONNECTION: u8 = 0x00;
const INTERRUPT: u8 = 0x03;

/// The link state a notification `n` gives, or `None` for another.
pub fn connected(n: &[u8]) -> Option<bool> {
    let h = n.get(..8)?;
    (h[0] == FROM_INTERFACE_CLASS && h[1] == NETWORK_CONNECTION).then(|| h[2] != 0 || h[3] != 0)
}

/// The first interrupt IN endpoint of interface `number`, alternate 0.
pub fn notify_ep(raw: &[u8], number: u8) -> Option<Notify> {
    let mut inside = false;
    walk(raw).find_map(|d| {
        if d[1] == INTERFACE && d.len() >= 4 {
            inside = d[2] == number && d[3] == 0;
        }
        let ours = inside && d[1] == ENDPOINT && d.len() >= 7;
        (ours && d[2] & 0x80 != 0 && d[3] & 0x3 == INTERRUPT).then(|| Notify {
            address: d[2],
            max_packet: u16::from_le_bytes([d[4], d[5]]),
            interval: d[6],
        })
    })
}
