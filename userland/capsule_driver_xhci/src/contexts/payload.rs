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

//! What an interrupt endpoint moves in one service interval (xHCI 1.2
//! section 6.2.3.8, USB 2.0 section 5.7.3, USB 3.2 section 4.4.7): one
//! packet, or on a high-bandwidth high-speed endpoint (wMaxPacketSize bits
//! 12:11) or a SuperSpeed one bursting (its companion's bMaxBurst) up to
//! three. Taking bits 10:0 alone, as this did, gave such an endpoint a
//! third of its bandwidth and a buffer it babbled past.

use super::ep0::{is_superspeed, SPEED_HIGH};

const MAX_PACKET: u16 = 1024;
const MAX_EXTRA: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Payload {
    /// wMaxPacketSize bits 10:0.
    pub max_packet: u16,
    /// Packets past the first in one interval: the Endpoint Context's Max
    /// Burst Size for both kinds (xHCI 1.2 section 6.2.3, as recalled,
    /// unverified).
    pub max_burst: u8,
    /// Max ESIT Payload: every packet of the interval.
    pub esit: u16,
}

/// From the descriptor's `w_max_packet` and, on SuperSpeed, `ss_burst`.
/// `None` for an endpoint past the specification, refused before the
/// controller sees it.
pub fn interrupt_payload(
    speed: u8,
    usb3: bool,
    w_max_packet: u16,
    ss_burst: u8,
) -> Option<Payload> {
    let max_packet = w_max_packet & 0x07FF;
    let max_burst = if is_superspeed(speed, usb3) {
        ss_burst
    } else if speed == SPEED_HIGH {
        (w_max_packet >> 11 & 0x3) as u8
    } else {
        0
    };
    let esit = max_packet * (max_burst.min(MAX_EXTRA) as u16 + 1);
    let fits = (1..=MAX_PACKET).contains(&max_packet) && max_burst <= MAX_EXTRA;
    fits.then_some(Payload { max_packet, max_burst, esit })
}
