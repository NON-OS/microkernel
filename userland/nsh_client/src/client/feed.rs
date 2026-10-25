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

//! Feeding the server's bytes in. Frames are cut out one at a time, each
//! judged against the stage the client is in when its length arrives. The
//! first refusal ends the session; once keys exist, the server is told the
//! one reason code that refusal allows, and otherwise nothing.

use super::Client;
use crate::{Attester, Rand, Refusal, UserSigner};
use nsh_wire::{split, Expect, Kind};

impl<A: Attester, S: UserSigner> Client<A, S> {
    /// Take bytes from the server. Events go to `events`, bytes to send to
    /// `take_output`. After an `Err`, the session is over: send what output
    /// is left, then close.
    pub fn feed(&mut self, mut input: &[u8], rand: Rand<'_>) -> Result<(), Refusal> {
        while !input.is_empty() {
            let Some(expect) = self.stage.expect() else { return Err(Refusal::Ended) };
            let step = match self.framer.fill(&mut input, expect) {
                Ok(None) => return Ok(()),
                Ok(Some((len, bytes))) => self.frame(expect, len, &bytes, rand),
                Err(e) => Err(Refusal::Wire(e)),
            };
            if let Err(r) = step {
                return Err(self.refuse(r));
            }
        }
        Ok(())
    }

    fn frame(
        &mut self,
        expect: Expect,
        len: [u8; 4],
        bytes: &[u8],
        rand: Rand<'_>,
    ) -> Result<(), Refusal> {
        if !expect.sealed() {
            return self.server_hello(bytes);
        }
        let mut inner = self.open(len, bytes)?;
        let done = match split(&inner, expect) {
            Ok((kind, body)) => self.sealed(kind, body, rand),
            Err(e) => Err(Refusal::Wire(e)),
        };
        nonos_hash::wipe(&mut inner);
        done
    }

    fn sealed(&mut self, kind: Kind, body: &[u8], rand: Rand<'_>) -> Result<(), Refusal> {
        match self.stage.expect() {
            Some(Expect::HostProof) => self.host_proof(kind, body, rand),
            Some(Expect::AuthResult) => self.auth_result(kind, body),
            Some(Expect::ClientSession) => self.session(kind, body),
            _ => Err(Refusal::Ended),
        }
    }
}
