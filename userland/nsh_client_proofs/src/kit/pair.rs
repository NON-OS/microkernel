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

//! A client and the test server side by side. A proof runs the good
//! handshake's steps by hand and changes one thing.

use super::attest::TestAttester;
use super::keys::user_key;
use super::proof::ProofSpec;
use super::rng::Det;
use super::server::Server;
use nsh_client::{Client, Config, Refusal, UserKey};
use nsh_crypto::Secret;
use nsh_wire::Kind;

pub use super::fleet::{config, policy};

pub struct Pair {
    pub client: Client<TestAttester, UserKey>,
    pub server: Server,
}

impl Pair {
    pub fn with(cfg: Config, attester: TestAttester) -> Pair {
        let client = Client::new(cfg, attester, user_key(), Secret::new([0x33; 32]), [0x52; 32]);
        Pair { client: client.expect("client starts"), server: Server::default() }
    }

    pub fn good() -> Pair {
        Pair::with(config(), TestAttester::default())
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), Refusal> {
        let mut r = Det::new(7);
        self.client.feed(bytes, &mut |b| r.fill(b))
    }

    /// Through the hellos: the client now waits for `HOST_PROOF`.
    pub fn to_proof(&mut self) {
        let ch = self.client.take_output();
        let sh = self.server.hello(&ch, |_| {});
        self.feed(&sh).expect("server hello");
    }

    /// `spec` as the server would seal it now; it joins its transcript.
    pub fn proof(&mut self, spec: &ProofSpec) -> Vec<u8> {
        let body = spec.bytes(&self.server);
        self.server.th.push(Kind::HostProof, &body);
        self.server.seal(Kind::HostProof, &body)
    }

    /// What the client has sent since the last call, opened.
    pub fn sent(&mut self) -> Vec<(u8, Vec<u8>)> {
        let out = self.client.take_output();
        self.server.open(&out).expect("client frames open")
    }
}
