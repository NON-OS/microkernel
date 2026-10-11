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

//! Taking back the slots the part has sent from, by the send-ring consumer
//! index it writes in the status block (tg3_tx). An index past the ring or
//! ahead of what was posted is the part's error and moves nothing.

use crate::constants::ring::TX_RING_SIZE;

use super::tx::TxRing;

impl TxRing {
    pub fn reclaim(&mut self, hw_cons: u16) {
        if hw_cons as usize >= TX_RING_SIZE {
            return;
        }
        let done = (hw_cons as usize + TX_RING_SIZE - self.cons as usize) % TX_RING_SIZE;
        if done <= self.in_flight() {
            self.cons = hw_cons;
        }
    }
}
