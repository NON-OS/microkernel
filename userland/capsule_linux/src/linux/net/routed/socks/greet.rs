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

//! The SOCKS5 greeting, answered before its request is read, and the
//! bound on what a conversation holds before its request is whole.

use alloc::vec::Vec;

use super::kinds::{Heard, Ver, REQUEST_MAX};
use super::parsed::Parsed;
use super::reply::{method, reply, GENERAL_FAILURE};
use super::talk::Talk;
use super::v5;

impl Talk {
    /// The greeting, when it is the one awaited: answered, and how many of
    /// the held bytes it took; Err when nothing more can be heard yet.
    pub(super) fn greet(&mut self, say: &mut Vec<u8>) -> Result<usize, Heard> {
        if self.greeted || self.held.first() != Some(&v5::VER) {
            return Ok(0);
        }
        let Parsed::Done(no_auth, n) = v5::greeting(&self.held) else {
            return Err(Heard::More);
        };
        say.extend_from_slice(&method(no_auth));
        if !no_auth {
            return Err(Heard::Closed);
        }
        self.greeted = true;
        Ok(n)
    }

    /// No whole request yet: more is awaited, unless what is held has
    /// reached REQUEST_MAX, which no request does. Only a SOCKS4 request,
    /// whose user id has no bound, or a SOCKS5 one after its greeting, can.
    pub(super) fn more(&self, took: usize, say: &mut Vec<u8>) -> (usize, Heard) {
        if self.held.len() < REQUEST_MAX {
            return (took, Heard::More);
        }
        let ver = if self.greeted { Ver::Five } else { Ver::Four };
        say.extend_from_slice(&reply(ver, GENERAL_FAILURE));
        (took, Heard::Closed)
    }
}
