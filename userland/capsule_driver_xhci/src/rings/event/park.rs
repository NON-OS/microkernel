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
//! waiter ever asks for, is not kept at all.
use super::issued_transfer::IssuedTransfer;
use super::state::{EventRing, PARKED};
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
    /// Keep `event` for its own waiter. With every place taken the oldest
    /// is given up, as the controller gives up events on a full ring. A
    /// Stopped event is not kept: the transfer it names was given up before
    /// its endpoint was stopped.
    pub fn park(&mut self, event: Trb) {
        if is_stopped(&event) {
            return;
        }
        match self.parked.iter().position(|p| p.is_none()) {
            Some(at) => self.parked[at] = Some(event),
            None => {
                self.parked.rotate_left(1);
                self.parked[PARKED - 1] = Some(event);
            }
        }
    }
    /// The parked event for `issued`, if one came.
    pub fn take_parked(&mut self, issued: IssuedTransfer) -> Option<Trb> {
        let at = self.parked.iter().position(|p| p.is_some_and(|e| issued.completed_by(&e)))?;
        self.parked[at].take()
    }
    /// Drop every parked event of endpoint `dci` on `slot`. Called when the
    /// endpoint is recovered: the transfers those events finish were given
    /// up, and their TRB addresses will be used again.
    pub fn forget_parked(&mut self, slot: u8, dci: u8) {
        for p in self.parked.iter_mut() {
            if p.is_some_and(|e| e.slot_id() == slot && e.endpoint_id() == dci) {
                *p = None;
            }
        }
    }
}
