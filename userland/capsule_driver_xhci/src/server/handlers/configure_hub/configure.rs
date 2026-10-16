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
use crate::contexts::{
    is_superspeed, route_depth, write_configure_hub_input, HubInfo, MAX_TIERS, SPEED_FULL,
    SPEED_HIGH,
};
use crate::controller::issue_configure_endpoint;
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;

/// Configure Endpoint with the hub fields for `slot`. A device that cannot
/// be a hub is refused before any command: one not addressed, a low speed
/// one (no hub is), and one five tiers down, whose ports would be a sixth.
/// TT Think Time and MTT go to the controller for a high-speed hub only;
/// on any other they are 0 (xHCI 1.2 section 6.2.2).
pub(super) fn configure(ctx: &mut Context, slot: u8, hub: HubInfo) -> XhciResult<()> {
    let d = &mut ctx.driver;
    let (csz, db, intr) =
        (d.layout.context_size, d.layout.doorbell_base, d.layout.primary_intr_base);
    let res =
        d.slots.resources_mut(slot, d.layout.max_slots).ok_or(XhciError::ControllerUnsupported)?;
    let depth = route_depth(res.topo.route).ok_or(XhciError::ControllerUnsupported)?;
    let superspeed = is_superspeed(res.speed, res.usb3);
    if depth >= MAX_TIERS || !(superspeed || matches!(res.speed, SPEED_FULL | SPEED_HIGH)) {
        return Err(XhciError::ControllerUnsupported);
    }
    let high_speed = !superspeed && res.speed == SPEED_HIGH;
    write_configure_hub_input(&res.input_context, &res.output_context, csz, hub, high_speed);
    let input = res.input_context.phys();
    issue_configure_endpoint(db, intr, &mut d.command_ring, &mut d.event_ring, input, slot)?;
    res.hub = Some(hub);
    Ok(())
}
