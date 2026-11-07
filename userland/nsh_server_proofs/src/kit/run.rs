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

//! A client against a server in memory. Every byte each way is kept, and
//! every event, so a proof sees the wire as well as what each side said.

use super::host::{config, Kernel, Keys};
use super::rng::Det;
use super::roles::Table;
use super::roles_build::grant::GrantB;
use super::roles_build::keys::K;
use super::roles_build::NOW;
use super::user::{attester, client_config, User};
use nsh_client::{Client, NonosAttester, Refusal};
use nsh_crypto::Secret;
use nsh_server::{Config, Event, Server};

pub type Host = Server<Kernel, Keys, Table>;

pub struct Run {
    pub client: Client<NonosAttester, User>,
    pub server: Host,
    pub c_events: Vec<nsh_client::Event>,
    pub s_events: Vec<Event>,
    /// Why the client ended the session, if it did.
    pub c_err: Option<Refusal>,
    pub c2s: Vec<u8>,
    pub s2c: Vec<u8>,
    pub now: u64,
    /// Answer every admitted login with `begin`, and every admitted
    /// escalation with `escalation_begun`.
    pub answer: bool,
    pub c_rng: Det,
    pub s_rng: Det,
}

impl Run {
    pub fn with(grant: &GrantB, cfg: Config, kernel: Kernel, keys: Keys, table: Table) -> Run {
        let mut c_rng = Det::new(0xC1);
        let ccfg = client_config(grant);
        let client =
            Client::new(ccfg, attester(), User(K::User), Secret::new(c_rng.bytes()), c_rng.bytes());
        let server = Server::new(cfg, kernel, keys, table, NOW).expect("a valid config");
        Run {
            client: client.expect("the client starts"),
            server,
            c_events: Vec::new(),
            s_events: Vec::new(),
            c_err: None,
            c2s: Vec::new(),
            s2c: Vec::new(),
            now: NOW,
            answer: true,
            c_rng,
            s_rng: Det::new(0x5E),
        }
    }

    pub fn new(grant: &GrantB, table: Table) -> Run {
        Run::with(grant, config(), Kernel::honest(), Keys::default(), table)
    }
}
