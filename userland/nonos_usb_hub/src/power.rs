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
//! Powering every port of a new hub (USB 2.0 section 11.11 and 11.23.2.1).
//! A hub without power switching ignores the request; its ports are powered
//! already. The wait for the power to be good is the hub's `ready_ms`, a
//! deadline its ports are looked at against: slept through here, up to
//! 510 ms, it held the walk, and every other hub's ports with it, each time
//! a hub, or a device posing as one, was plugged in.

use crate::descriptor::HubDescriptor;
use crate::host::Host;
use crate::line::Line;
use crate::request::{set_port_feature, PORT_POWER};

pub fn power_ports(h: &mut impl Host, slot: u8, desc: &HubDescriptor) {
    let mut refused = 0u32;
    for port in 1..=desc.ports {
        if !set_port_feature(h, slot, port, PORT_POWER) {
            refused += 1;
        }
    }
    if refused != 0 {
        Line::new()
            .text(b"hub slot ")
            .num(slot as u32)
            .text(b": ")
            .num(refused)
            .text(b" ports refused PORT_POWER")
            .say(h);
    }
}
