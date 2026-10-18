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

//! Naming armed transfers, keeping events, and handing them out.

use super::issued_transfer::IssuedTransfer;
use super::parked::{Parked, PARKED};
use crate::error::{XhciError, XhciResult};
use crate::trb::Trb;

impl Parked {
    /// Name `issued` as armed, before its doorbell rings, so its event is
    /// kept however many others arrive first. Refused when every place is
    /// taken: the endpoint is then not armed, rather than armed on an event
    /// the table might give up.
    pub fn arm(&mut self, issued: IssuedTransfer) -> XhciResult<()> {
        let free = self.armed.iter().position(Option::is_none);
        let at = free.ok_or(XhciError::TransferRingFull)?;
        self.armed[at] = Some(issued);
        Ok(())
    }

    /// Keep `event`. One an armed transfer waits for is always kept; any
    /// other may give up the oldest of its kind once `PARKED` are kept.
    pub fn park(&mut self, event: Trb) {
        let awaited = self.awaited(&event);
        let (oldest, unawaited) = self.oldest_unawaited();
        let free = self.kept.iter().position(Option::is_none);
        let at = match (free, oldest) {
            (Some(at), Some(old)) if !awaited && unawaited >= PARKED => self.give_up(old, at),
            (Some(at), _) => at,
            (None, Some(old)) => self.give_up(old, old),
            // Unreachable while each armed transfer has one event; refusing
            // the new event is still the safe answer.
            (None, None) => return self.given_up += 1,
        };
        self.seq += 1;
        self.kept[at] = Some((self.seq, event));
    }

    fn give_up(&mut self, old: usize, at: usize) -> usize {
        self.kept[old] = None;
        self.given_up += 1;
        at
    }

    /// The kept event of `issued`, if one came. Taking it also ends the
    /// arming: the transfer is finished. A transfer still on its way stays
    /// armed, or a flood before its event lands could give that event up.
    pub fn take(&mut self, issued: IssuedTransfer) -> Option<Trb> {
        let at = self.kept.iter().position(|k| k.is_some_and(|(_, e)| issued.completed_by(&e)))?;
        self.disarm(|a| a.same(&issued));
        self.kept[at].take().map(|(_, e)| e)
    }
}
