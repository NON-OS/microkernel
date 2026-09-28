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
use super::wire::method_reply;

impl Front {
    /// Drop `pid`'s conversation, ending its stream.
    pub(super) fn forget(&mut self, tunnel: &mut impl Tunnel, pid: u32) {
        if let Some(i) = self.held.iter().position(|h| h.0 == pid) {
            if let Some(id) = self.held.remove(i).1.stream() {
                tunnel.close(id);
            }
        }
    }

    pub(super) fn step(&mut self, tunnel: &mut impl Tunnel, pid: u32, data: &[u8]) -> Vec<u8> {
        let i = match self.held.iter().position(|h| h.0 == pid) {
            Some(i) => i,
            /* Refused, not queued, and nobody else's conversation evicted. */
            None if self.held.len() >= CALLERS_MAX => return encode(true, &method_reply(false)),
            None => {
                self.held.push((pid, Conv::default()));
                self.held.len() - 1
            }
        };
        let (over, out) = self.held[i].1.feed(tunnel, data);
        if over {
            self.forget(tunnel, pid);
        }
        encode(over, &out)
    }
}
