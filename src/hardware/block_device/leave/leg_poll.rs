// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! A leg's round (leg.rs): the lock, the next request posted, its reply.

use super::leg::Leg;
use super::rule::Poll;
use crate::services::lifecycle::transport::TransportError;
use crate::services::lifecycle::transport_nowait::{landed, post};

impl Leg {
    /// One round: take the lock if it is free, post the next request once,
    /// then look for its reply. Replies to an older request are read and
    /// dropped. A post the driver's inbox has no room for, or that could not
    /// be built, is tried again next round inside the budget; only a driver
    /// gone ends the leg.
    pub(crate) fn poll(&mut self) -> Poll {
        if self.held.is_none() {
            match self.wire.lock.try_lock() {
                Some(guard) => self.held = Some(guard),
                None => return Poll::Waiting,
            }
        }
        loop {
            let (id, request) = &self.requests[self.next];
            let gen = match self.posted {
                Some(gen) => gen,
                None => match post(request, self.wire.sender, self.wire.state) {
                    Ok(gen) => *self.posted.insert(gen),
                    Err(TransportError::Dead) => return Poll::Gone,
                    Err(_) => return Poll::Waiting,
                },
            };
            if !self.wire.state.is_alive() || self.wire.state.generation() != gen {
                return Poll::Gone;
            }
            let Some(status) = self.reply(*id, gen) else { return Poll::Waiting };
            if self.failed == 0 {
                self.failed = status;
            }
            self.next += 1;
            self.posted = None;
            if self.next == self.requests.len() {
                return Poll::Answered(self.failed);
            }
        }
    }

    /// The status of the reply to request `id`, if it has landed. A reply
    /// the generation refuses reads as no reply: the generation check above
    /// ends the leg on the next round.
    fn reply(&self, id: u32, gen: u64) -> Option<i32> {
        while let Some(reply) = landed(self.wire.reply_inbox, self.wire.state, gen) {
            let Ok(reply) = reply else { return None };
            match (self.wire.decode)(&reply.data) {
                Some(r) if r.request_id == id => return Some(r.status),
                _ => continue,
            }
        }
        None
    }
}
