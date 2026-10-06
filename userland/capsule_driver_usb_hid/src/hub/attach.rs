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

//! Bringing up a hub that was addressed and configured: its descriptor,
//! its depth (SuperSpeed), the controller told it is a hub, its status
//! endpoint claimed, and its ports powered (Linux `hub_configure`).

use crate::xhci::{alloc_transfer_ring, configure_hub};

use super::error::HubError;
use super::node::Hub;
use super::place::{speed_name, Place};
use super::power::power_ports;
use super::read::hub_descriptor;
use super::request::set_hub_depth;
use super::say::Line;
use super::status_endpoint::status_endpoint;

pub fn attach(xhci_port: u32, slot: u8, place: Place, raw: &[u8]) -> Result<Hub, HubError> {
    let ss = place.superspeed();
    let desc = hub_descriptor(xhci_port, slot, ss).ok_or(HubError::Descriptor)?;
    if ss && !set_hub_depth(xhci_port, slot, place.depth) {
        return Err(HubError::Depth);
    }
    let start = Line::new().text(b"hub slot ").num(slot as u32).text(b": ");
    if configure_hub(xhci_port, slot, desc.ports, desc.think_time).is_err() {
        start.text(b"the controller driver was not told this is a hub (op 0x21 refused); low and full speed devices below it will not work").say();
    }
    if let Some(ep) = status_endpoint(raw) {
        let _ = alloc_transfer_ring(xhci_port, slot, ep.address, ep.max_packet, ep.interval);
    }
    power_ports(xhci_port, slot, &desc);
    Line::new()
        .text(b"root port ")
        .num(place.root_port as u32)
        .text(b": ")
        .text(speed_name(place.speed))
        .text(b" hub in slot ")
        .num(slot as u32)
        .text(b", tier ")
        .num(place.depth as u32 + 1)
        .text(b", ")
        .num(desc.ports_named as u32)
        .text(b" ports, powered")
        .say();
    Ok(Hub::new(slot, place, desc.ports))
}
