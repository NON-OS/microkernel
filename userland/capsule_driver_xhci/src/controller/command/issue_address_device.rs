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
//! over IPC well inside it. Some devices want 10 ms
//! here (as observed, unverified), and this waits that long.
use nonos_libc::mk_idle_ms;

use super::run_command::run_command_within;
use crate::error::{XhciError, XhciResult};
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::trb::commands::address_device_command;

/// The settle time after SET_ADDRESS (as observed, unverified).
pub const SET_ADDRESS_SETTLE_MS: u64 = 10;
/// A device has 50 ms to finish SET_ADDRESS (USB 2.0 section 9.2.6.3);
/// slower ones are given 500 ms (as observed, unverified). One that NAKs its status stage
/// for good is given up then, not after the 5 s other commands get: the
/// server serves one request at a time, so that wait stalled every
/// keyboard and disk on the machine.
pub const ADDRESS_TIMEOUT_MS: u64 = 500;

pub fn issue_address_device(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    input_context_phys: u64,
    slot_id: u8,
) -> XhciResult<()> {
    let trb = address_device_command(cmd_ring.cycle() != 0, input_context_phys, slot_id);
    let (db, intr) = (doorbell_base, intr_base);
    let completion = run_command_within(db, intr, cmd_ring, evt_ring, trb, ADDRESS_TIMEOUT_MS)?;
    if completion.slot_id != slot_id {
        return Err(XhciError::UnexpectedCompletionSlot);
    }
    let _ = mk_idle_ms(SET_ADDRESS_SETTLE_MS);
    Ok(())
}
