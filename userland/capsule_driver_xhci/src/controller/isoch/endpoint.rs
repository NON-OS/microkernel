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

//! An isoch endpoint descriptor turned into what the controller is given.

use crate::contexts::{is_superspeed, isoch_interval, SPEED_HIGH};

/// One isoch endpoint. `interval` is the controller's exponent
/// (`isoch_interval`); `max_burst` the packets per burst less one (the
/// SuperSpeed companion's bMaxBurst, or a high-bandwidth high-speed
/// endpoint's additional transactions); `mult` the SuperSpeed bursts per
/// interval less one, 0 elsewhere.
#[derive(Clone, Copy)]
pub struct IsochEndpoint {
    pub dci: u8,
    pub ring_phys: u64,
    pub max_packet: u16,
    pub max_burst: u8,
    pub mult: u8,
    pub interval: u8,
}

impl IsochEndpoint {
    /// The most the endpoint moves in one service interval (Max ESIT
    /// Payload): what one TD may carry.
    pub fn esit_payload(&self) -> u32 {
        self.max_packet as u32 * (self.max_burst as u32 + 1) * (self.mult as u32 + 1)
    }
}

/// The endpoint the controller is given for this speed (xHCI 1.2 6.2.3 as recalled, unverified:
/// Max Burst Size and Mult, from wMaxPacketSize or the SuperSpeed companion).
pub fn isoch_endpoint(
    speed: u8,
    usb3: bool,
    dci: u8,
    b_interval: u8,
    ss: (u8, u8),
    mps: u16,
) -> IsochEndpoint {
    let superspeed = is_superspeed(speed, usb3);
    let (max_burst, mult) = if superspeed {
        (ss.0.min(15), ss.1 & 0x3)
    } else if speed == SPEED_HIGH {
        (((mps >> 11) & 0x3) as u8, 0)
    } else {
        (0, 0)
    };
    let max_packet = mps & 0x07FF;
    let interval = isoch_interval(speed, usb3, b_interval);
    IsochEndpoint { dci, ring_phys: 0, max_packet, max_burst, mult, interval }
}
