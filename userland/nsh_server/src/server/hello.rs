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

//! `CLIENT_HELLO`, the one plaintext frame the server receives. A hello of
//! another version is told `ABORT(VERSION)` in plaintext; any other broken
//! hello is closed without a word, since nothing else can be said before
//! keys exist. The answer is `SERVER_HELLO`, the handshake keys, and then
//! `HOST_PROOF` under them.

use super::busy::plain_abort;
use super::stage::Stage;
use super::Server;
use crate::{Attestation, HostSigner, Rand, Roles, Why};
use nsh_crypto::{encapsulate, handshake_keys, Keys, Opener, Sealer, Secret};
use nsh_wire::hello::RANDOM_LEN;
use nsh_wire::{len_field, split, ClientHello, Expect, Kind, ServerHello, WireError};
use nsh_wire::{Reason, SERVER_HELLO_LEN};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn client_hello(&mut self, frame: &[u8], rand: Rand<'_>) -> Result<(), Why> {
        let (_, body) = split(frame, Expect::ClientHello).map_err(Why::Wire)?;
        let hello = match ClientHello::parse(body) {
            Ok(h) => h,
            Err(WireError::Version) => {
                self.out.extend_from_slice(&plain_abort(Reason::Version));
                return Err(Why::Wire(WireError::Version));
            }
            Err(e) => return Err(Why::Wire(e)),
        };
        let mut random = [0u8; RANDOM_LEN];
        let mut seed = [0u8; 64];
        if !rand(&mut random) || !rand(&mut seed) {
            nonos_hash::wipe(&mut seed);
            return Err(Why::Rand);
        }
        let (ct, ss) = encapsulate(hello.kem_ek, Secret::take(&mut seed))
            .map_err(|_| Why::Wire(WireError::Value))?;
        let mut sh = [0u8; SERVER_HELLO_LEN];
        let n = ServerHello { random: &random, kem_ct: &ct }.encode(&mut sh).map_err(Why::Wire)?;
        let len = len_field(n, Expect::ServerHello).map_err(Why::Wire)?;
        self.th.push(Kind::ClientHello, body);
        self.th.push(Kind::ServerHello, &sh[..n]);
        self.th_hello = self.th.digest();
        let (Keys { c2s, s2c }, hs) = handshake_keys(ss, &self.th_hello);
        self.sealer = Some(Sealer::new(s2c));
        self.opener = Some(Opener::new(c2s));
        self.hs = Some(hs);
        self.out.extend_from_slice(&len);
        self.out.push(Kind::ServerHello.byte());
        self.out.extend_from_slice(&sh[..n]);
        self.stage = Stage::UserAuth;
        self.host_proof(rand)
    }
}
