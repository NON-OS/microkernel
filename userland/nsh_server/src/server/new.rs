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

//! A server for one connection, from the moment it is accepted.

use super::stage::Stage;
use super::Server;
use crate::framer::Framer;
use crate::{Attestation, BadConfig, Config, HostSigner, Roles};
use alloc::vec::Vec;
use nsh_crypto::Transcript;

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// `now_ms` is the host clock when the connection was accepted; the
    /// handshake must finish within `cfg.handshake_ms` of it.
    pub fn new(
        cfg: Config,
        attest: A,
        signer: S,
        roles: R,
        now_ms: u64,
    ) -> Result<Self, BadConfig> {
        if !cfg.valid() {
            return Err(BadConfig);
        }
        Ok(Server {
            cfg,
            attest,
            signer,
            roles,
            stage: Stage::ClientHello,
            started_ms: now_ms,
            th: Transcript::default(),
            th_hello: [0; 64],
            doc_hash: [0; 64],
            hs: None,
            sealer: None,
            opener: None,
            framer: Framer::default(),
            out: Vec::new(),
            events: Vec::new(),
            admitted: None,
            live: None,
        })
    }
}
