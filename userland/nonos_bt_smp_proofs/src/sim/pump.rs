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

//! Moving PDUs between the two sides, one at a time, the responder's
//! inbox first, so each side sees the other's PDUs in order.

use super::run::Run;
use nonos_bt_smp::pairing::Outcome;

/// Passkey entry takes about 90 PDUs; anything near this is a loop.
const MAX_PDUS: usize = 400;

impl Run {
    /// Runs until no PDU is left in flight; the last outcome.
    pub fn go(&mut self) -> Option<Outcome> {
        for _ in 0..MAX_PDUS {
            if let Some(p) = self.to_peer.pop_front() {
                let replies = self.peer.on(&p);
                self.to_init.extend(replies);
            } else if let Some(p) = self.to_init.pop_front() {
                self.now += 10;
                let step = self.init.on_pdu(&p, &mut self.rng, self.now);
                self.take(step);
            } else {
                return self.outcomes.last().copied();
            }
        }
        panic!("pairing did not settle within {MAX_PDUS} PDUs");
    }
}
