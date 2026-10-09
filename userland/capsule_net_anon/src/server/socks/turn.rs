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

//! One turn of a caller's conversation, and its end.

extern crate alloc;

use alloc::vec::Vec;

use super::conv::Conv;
use super::front::{Front, CALLERS_MAX};
use super::reply::encode;
use super::tunnel::Tunnel;
use super::who::{pid_of, Who, STREAMS_PER_CALLER};
use super::wire::method_reply;

impl Front {
    /// Drop `w`'s conversation, ending its stream.
    pub(super) fn forget(&mut self, tunnel: &mut impl Tunnel, w: Who) {
        if let Some(i) = self.held.iter().position(|h| h.0 == w) {
            if let Some(id) = self.held.remove(i).1.stream() {
                tunnel.close(id);
            }
        }
    }

    /// One turn of `w`'s conversation, its answer bringing back at most
    /// `room` stream bytes.
    pub(super) fn step(
        &mut self,
        tunnel: &mut impl Tunnel,
        w: Who,
        data: &[u8],
        room: usize,
    ) -> Vec<u8> {
        let i = match self.held.iter().position(|h| h.0 == w) {
            Some(i) => i,
            /* Refused, not queued, and nobody else's conversation evicted:
             * the table is full, or this caller holds its whole share. */
            None if self.held.len() >= CALLERS_MAX || self.share(w) >= STREAMS_PER_CALLER => {
                return encode(true, &method_reply(false));
            }
            None => {
                self.held.push((w, Conv::default()));
                self.held.len() - 1
            }
        };
        let (over, out) = self.held[i].1.feed(tunnel, data, room);
        if over {
            self.forget(tunnel, w);
        }
        encode(over, &out)
    }

    /// How many conversations `w`'s caller holds.
    fn share(&self, w: Who) -> usize {
        self.held.iter().filter(|h| pid_of(h.0) == pid_of(w)).count()
    }
}
