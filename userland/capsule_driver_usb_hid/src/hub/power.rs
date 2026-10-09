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

//! Powering every port of a new hub and waiting, on the clock, for the
//! power to be good (USB 2.0 section 11.11 and 11.23.2.1, Linux
//! `hub_power_on`). A hub without power switching ignores the request; its
//! ports are powered already.

use nonos_libc::mk_idle_ms;

use super::descriptor::HubDescriptor;
use super::request::{set_port_feature, PORT_POWER};
use super::say::Line;

pub fn power_ports(xhci_port: u32, slot: u8, desc: &HubDescriptor) {
    let mut refused = 0u32;
    for port in 1..=desc.ports {
        if !set_port_feature(xhci_port, slot, port, PORT_POWER) {
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
            .say();
    }
    let _ = mk_idle_ms(desc.power_on_ms as u64);
}
