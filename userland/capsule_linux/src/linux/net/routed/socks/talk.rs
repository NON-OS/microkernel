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

//! One conversation with the endpoint, up to its CONNECT. A write is taken
//! only as far as the request reaches: bytes past a whole request are left
//! to the guest's next write, which goes to the stream once it is open.

use alloc::vec::Vec;

use super::kinds::{Heard, Ver, REQUEST_MAX};
use super::parsed::Parsed;
use super::reply::reply;
use super::{v4, v5};

#[derive(Default)]
pub struct Talk {
    pub(super) held: Vec<u8>,
    /// The SOCKS5 greeting was answered and its request is awaited.
    pub(super) greeted: bool,
}

impl Talk {
    /// Hear the front of `bytes`: how many it took, and what came of them.
    /// What the guest is told is put in `say`.
    pub fn hear(&mut self, bytes: &[u8], say: &mut Vec<u8>) -> (usize, Heard) {
        let before = self.held.len();
        let took = bytes.len().min(REQUEST_MAX - before);
        self.held.extend_from_slice(&bytes[..took]);
        let used = match self.greet(say) {
            Ok(used) => used,
            Err(Heard::More) => return self.more(took, say),
            Err(heard) => return (took, heard),
        };
        let rest = &self.held[used..];
        let (ver, got) = match (self.greeted, rest.first()) {
            (true, _) => (Ver::Five, v5::request(rest)),
            (false, Some(&v4::VER)) => (Ver::Four, v4::request(rest)),
            (false, Some(_)) => return (took, Heard::Closed),
            (false, None) => return self.more(took, say),
        };
        match got {
            Parsed::Done(c, n) => {
                self.held.clear();
                ((used + n).saturating_sub(before), Heard::Connect(ver, c))
            }
            Parsed::Need => {
                self.held.drain(..used);
                self.more(took, say)
            }
            Parsed::Refused(r) => {
                say.extend_from_slice(&reply(ver, r.code()));
                (took, Heard::Closed)
            }
            Parsed::Bad => (took, Heard::Closed),
        }
    }
}
