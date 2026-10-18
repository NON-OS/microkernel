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
//! Transfer events that arrive while a different transfer or command is
//! being waited for. One endpoint's completion must not be consumed by
//! another's wait: an interrupt-IN report finishing during a bulk transfer
//! is kept here until its own poll asks for it.
//!
//! A parked event is matched on its TRB's address, slot and endpoint, and a
//! transfer ring is 64 TRBs: an event nobody takes is the completion of
//! whatever TRB lands at that address one lap later. A USB stick whose bulk
//! transfer timed out had its Stopped event, or the late completion, parked
//! this way, and a later CBW or data phase on that ring was answered with it
//! at once, out of phase with the device. Events for an endpoint are now
//! dropped when the endpoint is recovered, and a Stopped event, which no
//! waiter ever asks for, is not kept at all. What a full table gives up is
//! `parked`'s rule.
use super::issued_transfer::IssuedTransfer;
use super::state::EventRing;
use crate::error::XhciResult;
use crate::trb::Trb;

/// The completion codes of a TRB the endpoint was on when Stop Endpoint
/// stopped it (xHCI 1.2 section 6.4.5): Stopped, Stopped - Length Invalid
/// and Stopped - Short Packet.
const CC_STOPPED: u8 = 26;
const CC_STOPPED_LENGTH_INVALID: u8 = 27;
const CC_STOPPED_SHORT_PACKET: u8 = 28;

/// Whether `event` reports a TRB an endpoint was stopped on.
fn is_stopped(event: &Trb) -> bool {
    matches!(
        event.completion_code(),
        CC_STOPPED | CC_STOPPED_LENGTH_INVALID | CC_STOPPED_SHORT_PACKET
    )
}

impl EventRing {
    /// Keep `event` for its own waiter, unless it is a Stopped event: its
    /// transfer was given up before the endpoint was stopped.
    pub fn park(&mut self, event: Trb) {
        if !is_stopped(&event) {
            self.parked.park(event);
        }
    }
    /// Name `issued` as armed before its doorbell rings; see `Parked::arm`.
    pub fn arm(&mut self, issued: IssuedTransfer) -> XhciResult<()> {
        self.parked.arm(issued)
    }
    /// The parked event for `issued`, if one came.
    pub fn take_parked(&mut self, issued: IssuedTransfer) -> Option<Trb> {
        self.parked.take(issued)
    }
    /// Drop endpoint `dci` of `slot`'s arming and parked events when it is
    /// recovered: those transfers were given up, their addresses come again.
    pub fn forget_parked(&mut self, slot: u8, dci: u8) {
        self.parked.forget(slot, dci);
    }
}
