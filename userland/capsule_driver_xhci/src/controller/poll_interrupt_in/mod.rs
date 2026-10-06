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
mod arm;
mod scan;
use crate::error::{XhciError, XhciResult};
use crate::rings::event::EventRing;
use crate::slots::SlotResources;
pub enum IntrPoll {
    Complete(u16),
    Pending,
}
/// Poll the interrupt-IN endpoint at `dci` of `res` for one report of up to
/// `length` bytes, arming it first if nothing is armed. A report that ended
/// in an error (STALL, babble, transaction error) is
/// `TransferCompletionFailed`: the endpoint is halted and must be recovered
/// before it is armed again.
pub fn poll_interrupt_in(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    res: &mut SlotResources,
    dci: u8,
    length: u16,
) -> XhciResult<IntrPoll> {
    let slot = res.slot_id;
    let ep = res.interrupt_mut(dci).ok_or(XhciError::ControllerUnsupported)?;
    if ep.armed.is_none() {
        arm::arm(doorbell_base, slot, ep, length)?;
    }
    scan::scan(intr_base, evt_ring, slot, ep, length)
}
