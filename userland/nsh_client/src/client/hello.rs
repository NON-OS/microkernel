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

//! `SERVER_HELLO`, the one plaintext frame the client receives. It may be
//! refused only by `ABORT(VERSION)` or `ABORT(BUSY)`, since nothing else
//! can be said before keys exist. After it come the handshake keys, and
//! the X-Wing key pair, used once, is gone.

use super::stage::Stage;
use super::Client;
use crate::{Attester, Refusal, UserSigner};
use nsh_crypto::{handshake_keys, Keys, Opener, Sealer};
use nsh_wire::{plain_abort, split, Expect, Kind, ServerHello, WireError};

impl<A: Attester, S: UserSigner> Client<A, S> {
    pub(super) fn server_hello(&mut self, frame: &[u8]) -> Result<(), Refusal> {
        let (kind, body) = split(frame, Expect::ServerHello).map_err(Refusal::Wire)?;
        match kind {
            Kind::ServerHello => {}
            Kind::Abort => return Err(Refusal::Aborted(plain_abort(body).map_err(Refusal::Wire)?)),
            _ => return Err(Refusal::Wire(WireError::Order)),
        }
        let hello = ServerHello::parse(body).map_err(Refusal::Wire)?;
        let kem = self.kem.take().ok_or(Refusal::Ended)?;
        self.th.push(Kind::ServerHello, body);
        self.th_hello = self.th.digest();
        self.bound.transcript = Some(self.th_hello);
        let ss = kem.decapsulate(hello.kem_ct);
        let (Keys { c2s, s2c }, hs) = handshake_keys(ss, &self.th_hello);
        self.sealer = Some(Sealer::new(c2s));
        self.opener = Some(Opener::new(s2c));
        self.hs = Some(hs);
        self.stage = Stage::HostProof;
        Ok(())
    }
}
