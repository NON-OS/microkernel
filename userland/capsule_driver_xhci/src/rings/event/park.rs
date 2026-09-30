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
use super::state::{EventRing, PARKED};
use crate::trb::Trb;
impl EventRing {
    /// Keep `event` for its own waiter. With every place taken the oldest
    /// is given up, as the controller gives up events on a full ring.
    pub fn park(&mut self, event: Trb) {
        match self.parked.iter().position(|p| p.is_none()) {
            Some(at) => self.parked[at] = Some(event),
            None => {
                self.parked.rotate_left(1);
                self.parked[PARKED - 1] = Some(event);
            }
        }
    }
    /// The parked event for the TRB issued at `issued_phys`, if one came.
    pub fn take_parked(&mut self, issued_phys: u64) -> Option<Trb> {
        let at = self
            .parked
            .iter()
            .position(|p| p.is_some_and(|e| e.get_pointer() & !0xF == issued_phys & !0xF))?;
        self.parked[at].take()
    }
}
