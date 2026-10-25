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
//! Discovery with one of the peripheral's answers changed on the way.

use super::run::{Run, MAX_PDUS};
use nonos_bt_hogp::host::{Outcome, TIMEOUT_MS};

impl Run {
    /// As `go`, with the peripheral's answer number `n` (from 0) passed
    /// through `lie` first; then, should the host still wait, its ATT
    /// transaction timeout. The last outcome, and how many answers came.
    pub fn go_lying(
        &mut self,
        n: usize,
        mut lie: impl FnMut(&mut Vec<u8>),
    ) -> (Option<Outcome>, usize) {
        let mut answers = 0;
        for _ in 0..MAX_PDUS {
            let Some(pdu) = self.to_peer.pop_front() else {
                self.now += TIMEOUT_MS;
                let step = self.host.poll(self.now);
                self.take(step);
                return (self.outcomes.last().copied(), answers);
            };
            if let Some(mut answer) = self.peer.on(&pdu) {
                if answers == n {
                    lie(&mut answer);
                }
                answers += 1;
                self.deliver(&answer);
            }
        }
        panic!("discovery did not settle within {MAX_PDUS} PDUs");
    }
}
