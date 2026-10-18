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

//! The end of armings: a transfer whose event came straight off the ring,
//! and a slot disabled with endpoints still armed.

use super::issued_transfer::IssuedTransfer;
use super::state::EventRing;

/// The highest Device Context Index (xHCI 1.2 section 4.5.1).
const MAX_DCI: u8 = 31;

impl EventRing {
    /// `issued` is done: its event was taken at the dequeue pointer, not
    /// from the parked table, or it was never queued. It is no longer armed.
    pub fn finished(&mut self, issued: IssuedTransfer) {
        self.parked.disarm(|a| a.same(&issued));
    }
    /// Forget every endpoint of `slot` once it is disabled. An interrupt
    /// endpoint stays armed between polls, so an unplugged keyboard left its
    /// arming behind, and enough replugs filled the table.
    pub fn forget_slot(&mut self, slot: u8) {
        for dci in 1..=MAX_DCI {
            self.parked.forget(slot, dci);
        }
    }
}
