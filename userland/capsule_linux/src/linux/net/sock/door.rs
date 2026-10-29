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

//! The door: the client end of a loopback stream that the personality holds
//! for the capsule SERVE_ONE serves (net/serve_one). It joins the guest's
//! listener as a loopback client would, bound to 127.0.0.1 and an ephemeral
//! port, so the guest accepts, reads, writes, polls and closes its end as
//! any other; no guest holds the door end itself.

use crate::linux::abi::errno::{EAGAIN, ECONNREFUSED, EPIPE};

use super::kinds::Link;
use super::table::Socks;
use super::types::{Addr, Domain, Proto};

impl Socks {
    /// A door end joined to the listener at `at`, its server end queued for
    /// accept: ECONNREFUSED with no listener, EAGAIN with a full queue.
    pub fn open_door(&mut self, at: Addr) -> Result<u32, i64> {
        let end = self.open(Domain::Inet, Proto::Stream, None);
        let failed = match self.autobind(end).map(|()| self.link(end, at)) {
            Ok(Link::Done) => return Ok(end),
            Ok(Link::Refused) => ECONNREFUSED,
            Ok(Link::Full) => EAGAIN,
            Err(e) => e,
        };
        self.free(end, false);
        Err(failed)
    }

    /// Bytes for the guest, as many as its end has room for: 0 when it is
    /// full, EPIPE once it is gone.
    pub fn door_put(&mut self, end: u32, bytes: &[u8]) -> Result<usize, i64> {
        if self.get(end).is_none_or(|s| s.peer.is_none()) {
            return Err(EPIPE);
        }
        match self.write(end, bytes) {
            Err(EAGAIN) => Ok(0),
            took => took,
        }
    }

    /// The door end let go. The guest reads end of file, not the reset that
    /// bytes left unread would send: the capsule chose to close.
    pub fn door_close(&mut self, end: u32) {
        if let Some(s) = self.get_mut(end) {
            s.rx.clear();
        }
        self.free(end, false);
    }
}
