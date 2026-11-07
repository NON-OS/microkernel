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

//! The SOCKS5 greeting and the username and password it may choose, each
//! answered before what follows is read, and the bound on what a
//! conversation holds before its request is whole.

use alloc::vec::Vec;

use super::kinds::{Heard, Ver, REQUEST_MAX};
use super::parsed::Parsed;
use super::reply::{method, reply, GENERAL_FAILURE};
use super::talk::{Stage, Talk};
use super::v5::Method;
use super::{sign_in, v5};

impl Talk {
    /// The greeting, and the username and password it chose, when they are
    /// awaited: answered, and how many of the held bytes they took; Err when
    /// nothing more can be heard yet.
    pub(super) fn greet(&mut self, say: &mut Vec<u8>) -> Result<usize, Heard> {
        let mut used = 0;
        if self.stage == Stage::Greeting {
            if self.held.first() != Some(&v5::VER) {
                return Ok(0);
            }
            let Parsed::Done(chosen, n) = v5::greeting(&self.held) else {
                return Err(Heard::More);
            };
            say.extend_from_slice(&method(chosen));
            self.stage = match chosen {
                Method::NoAuth => Stage::Request,
                Method::UserPassword => Stage::SignIn,
                Method::NoneAcceptable => return Err(Heard::Closed),
            };
            used = n;
        }
        if self.stage == Stage::SignIn {
            used += self.sign_in(used, say)?;
        }
        Ok(used)
    }

    /// The pair past the `from` held bytes already answered: how many it
    /// took. Until it is whole, what was answered is let go.
    fn sign_in(&mut self, from: usize, say: &mut Vec<u8>) -> Result<usize, Heard> {
        match sign_in::credentials(self.held.get(from..).unwrap_or_default()) {
            Parsed::Done((), n) => {
                say.extend_from_slice(&sign_in::status(true));
                self.stage = Stage::Request;
                Ok(n)
            }
            Parsed::Need => {
                self.held.drain(..from);
                Err(Heard::More)
            }
            Parsed::Refused(_) | Parsed::Bad => {
                say.extend_from_slice(&sign_in::status(false));
                Err(Heard::Closed)
            }
        }
    }

    /// No whole message yet: more is awaited, unless what is held has
    /// reached REQUEST_MAX, which no message does. Only a SOCKS4 request,
    /// whose user id has no bound, or a SOCKS5 one after its greeting, can.
    pub(super) fn more(&self, took: usize, say: &mut Vec<u8>) -> (usize, Heard) {
        if self.held.len() < REQUEST_MAX {
            return (took, Heard::More);
        }
        match self.stage {
            Stage::Greeting => say.extend_from_slice(&reply(Ver::Four, GENERAL_FAILURE)),
            Stage::SignIn => say.extend_from_slice(&sign_in::status(false)),
            Stage::Request => say.extend_from_slice(&reply(Ver::Five, GENERAL_FAILURE)),
        }
        (took, Heard::Closed)
    }
}
