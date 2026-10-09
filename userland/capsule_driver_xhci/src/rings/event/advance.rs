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
use super::state::EventRing;
use crate::constants::EVENT_RING_SEGMENT_TRBS;
impl EventRing {
    /// Step past the event just read, leaving its TRB as the controller wrote
    /// it. Its cycle bit is the one this lap consumed, so once the dequeue
    /// pointer wraps and the consumer cycle flips it reads as not yet written,
    /// which is how the controller's next write to the slot is told apart.
    /// Clearing the TRB would set its cycle bit to zero: on every second lap
    /// that is the bit being looked for, and each cleared slot would read as
    /// a new event ahead of the controller.
    pub fn advance(&mut self) {
        self.drained_total = self.drained_total.wrapping_add(1);
        self.dequeue_index += 1;
        if self.dequeue_index == EVENT_RING_SEGMENT_TRBS {
            self.dequeue_index = 0;
            self.consumer_cycle ^= 1;
        }
    }
}
