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

//! The two stages that follow the stream: waiting for the exit to connect,
//! then carrying bytes both ways.

extern crate alloc;

use alloc::vec::Vec;

use super::conv::Conv;
use super::rep::{rep_for_end, reply, REP_FAILURE, REP_OK};
use super::stage::{Stage, Step, OUT_MAX};
use super::tunnel::{Far, Tunnel, Unsent};

impl Conv {
    /* Success is answered only once the exit says it connected, so a caller
     * never starts TLS into a stream the exit refused. */
    pub(super) fn connecting(&mut self, t: &mut impl Tunnel, id: u16, out: &mut Vec<u8>) -> Step {
        let rep = match t.far(id) {
            Far::Opening => return Step::Wait,
            Far::Open => {
                out.extend_from_slice(&reply(REP_OK));
                self.stage = Stage::Relay(id);
                return Step::Next;
            }
            Far::Ended(reason) => rep_for_end(reason),
            Far::Gone => REP_FAILURE,
        };
        out.extend_from_slice(&reply(rep));
        Step::Over
    }

    /* `room` is how many stream bytes the answer may bring back: less than
     * OUT_MAX when it goes behind an answer the caller missed. */
    pub(super) fn relay(
        &mut self,
        t: &mut impl Tunnel,
        id: u16,
        out: &mut Vec<u8>,
        room: usize,
    ) -> Step {
        if !self.unsent.is_empty() {
            match t.send(id, &self.unsent) {
                Ok(sent) => {
                    self.unsent.drain(..sent);
                }
                Err(Unsent::Blocked) => {}
                /* Nothing more can go, but what already arrived is still the
                 * caller's, so the conversation ends only once that is read. */
                Err(Unsent::Over) => self.unsent.clear(),
            }
        }
        let room = room.min(OUT_MAX).saturating_sub(out.len());
        out.extend_from_slice(&t.take(id, room));
        match t.far(id) {
            Far::Opening | Far::Open => Step::Wait,
            Far::Ended(_) if t.waiting(id) > 0 => Step::Wait,
            Far::Ended(_) | Far::Gone => Step::Over,
        }
    }
}
