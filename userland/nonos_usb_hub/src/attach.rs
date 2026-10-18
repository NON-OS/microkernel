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
//! endpoint claimed, and its ports powered.

use crate::error::HubError;
use crate::host::Host;
use crate::line::Line;
use crate::node::Hub;
use crate::place::{speed_name, Place};
use crate::port_status::SPEED_HIGH;
use crate::power::power_ports;
use crate::read::hub_descriptor;
use crate::request::set_hub_depth;
use crate::status_endpoint::status_endpoint;

/// The hub in `slot` at `place`, whose configuration is `raw`.
pub fn attach(h: &mut impl Host, slot: u8, place: Place, raw: &[u8]) -> Result<Hub, HubError> {
    let ss = place.superspeed();
    let desc = hub_descriptor(h, slot, ss).ok_or(HubError::Descriptor)?;
    if ss && !set_hub_depth(h, slot, place.depth) {
        return Err(HubError::Depth);
    }
    // Only a high-speed hub has a TT; the bits are reserved on another
    // (USB 2.0 11.23.2.1, as recalled, unverified), so none is passed on.
    let think = if place.speed == SPEED_HIGH { desc.think_time } else { 0 };
    if let Err(status) = h.configure_hub(slot, desc.ports, think) {
        let line = Line::new().text(b"hub slot ").num(slot as u32).text(b": ");
        line.text(HubError::Configure(status).says()).say(h);
    }
    if let Some(ep) = status_endpoint(raw) {
        let _ = h.status_endpoint(slot, ep);
    }
    power_ports(h, slot, &desc);
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
        .say(h);
    let mut hub = Hub::new(slot, place, desc.ports);
    hub.ready_ms = (h.now_ms().max(0) as u64).saturating_add(u64::from(desc.power_on_ms));
    Ok(hub)
}
