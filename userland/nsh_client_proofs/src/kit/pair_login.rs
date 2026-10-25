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

//! The good handshake run through to `USER_AUTH`, and on to the session.

use super::pair::Pair;
use super::proof::ProofSpec;
use super::server::Server;
use nsh_client::Event;
use nsh_wire::Kind;

impl Pair {
    /// Up to `USER_AUTH`, checked by the server; gives its body.
    pub fn to_auth(&mut self) -> Vec<u8> {
        self.to_proof();
        let hp = self.proof(&ProofSpec::good());
        self.feed(&hp).expect("host proof");
        let sent = self.sent();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].0, Kind::UserAuth.byte());
        assert!(self.server.user_auth(&sent[0].1), "server accepts USER_AUTH");
        sent[0].1.clone()
    }

    pub fn login(&mut self) -> Vec<Event> {
        self.to_auth();
        let ok = self.server.auth_ok(&Server::auth_ok_body());
        self.feed(&ok).expect("auth ok");
        self.client.events()
    }
}
