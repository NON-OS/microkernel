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

//! Address Device, which has the controller send the device its
//! SET_ADDRESS (xHCI 1.2 section 4.6.5). The device may then ignore Setup
//! packets for its SetAddress recovery interval (USB 2.0 section 9.2.6.3,
//! 2 ms), and the next request, the class driver's descriptor read, came
//! over IPC well inside it. Linux's hub_port_init sleeps 10 ms here ("let
//! SET_ADDRESS settle, some device hardware wants it"), and so does this.
use nonos_libc::mk_idle_ms;

use super::run_command::run_command;
use crate::error::{XhciError, XhciResult};
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::trb::commands::address_device_command;

/// The settle time after SET_ADDRESS, as Linux gives it.
pub const SET_ADDRESS_SETTLE_MS: u64 = 10;

pub fn issue_address_device(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    input_context_phys: u64,
    slot_id: u8,
) -> XhciResult<()> {
    let trb = address_device_command(cmd_ring.cycle() != 0, input_context_phys, slot_id);
    let completion = run_command(doorbell_base, intr_base, cmd_ring, evt_ring, trb)?;
    if completion.slot_id != slot_id {
        return Err(XhciError::UnexpectedCompletionSlot);
    }
    let _ = mk_idle_ms(SET_ADDRESS_SETTLE_MS);
    Ok(())
}
