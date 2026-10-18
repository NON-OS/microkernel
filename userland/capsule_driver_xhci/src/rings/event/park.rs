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
//! is kept until its own poll asks for it.
//!
//! Only an armed TRB's event is kept (an interrupt report, the polled bulk
//! IN, an isoch batch's last TD), in the place its poll armed (`expect`).
//! Control and waited bulk transfers are waited for by the request that
//! issued them, so no other wait reads their events, save a
//! late one for a transfer given up, which no waiter asks for and which,
//! kept, would answer the TRB written at its address one lap later. The
//! list was eight events kept whatever they named, the oldest given up
//! when full: an interrupt report lost that way left its endpoint armed
//! for good, and its device silent. A place is now refused when all are
//! taken, before anything is armed, and nothing kept is ever given up.
use super::issued_transfer::IssuedTransfer;
use super::state::EventRing;
use crate::constants::{CC_RING_OVERRUN, CC_RING_UNDERRUN};
use crate::trb::Trb;

/// The completion codes of a TRB the endpoint was on when Stop Endpoint
/// stopped it (xHCI 1.2 section 6.4.5): Stopped, Stopped - Length Invalid
/// and Stopped - Short Packet. Not a completion: the transfer it names
/// was given up before its endpoint was stopped.
const CC_STOPPED: u8 = 26;
const CC_STOPPED_SHORT_PACKET: u8 = 28;

/// An armed TRB, and its event once read.
#[derive(Clone, Copy)]
pub struct Armed {
    pub(super) issued: IssuedTransfer,
    pub(super) event: Option<Trb>,
}

impl EventRing {
    /// Keep `event` for the poll of the armed TRB it completes; any other,
    /// a Stopped event, and an isoch ring that ran dry or overfilled, which
    /// names no TD, is nobody's and is dropped.
    pub fn park(&mut self, event: Trb) {
        let cc = event.completion_code();
        if (CC_STOPPED..=CC_STOPPED_SHORT_PACKET).contains(&cc)
            || matches!(cc, CC_RING_UNDERRUN | CC_RING_OVERRUN)
        {
            return;
        }
        let mine = |a: &&mut Armed| a.event.is_none() && a.issued.completed_by(&event);
        if let Some(a) = self.armed.iter_mut().flatten().find(mine) {
            a.event = Some(event);
        }
    }
    /// The event kept for `issued`, which is then no longer armed.
    pub fn take_parked(&mut self, issued: IssuedTransfer) -> Option<Trb> {
        let at = self.armed.iter().position(|a| a.is_some_and(|a| a.issued == issued))?;
        let event = self.armed[at]?.event?;
        self.armed[at] = None;
        Some(event)
    }
}
