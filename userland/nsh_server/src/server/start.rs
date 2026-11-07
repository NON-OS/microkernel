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

//! Starting an admitted session: a fresh id, `AUTH_OK`, the record's header,
//! and the session keys in place of the handshake keys and secret.

use super::admit::auth_ok;
use super::stage::Stage;
use super::{Live, Server};
use crate::record::{Header, Record};
use crate::{Attestation, Event, HostSigner, Rand, Roles, Why};
use alloc::vec::Vec;
use nsh_crypto::{session_keys, Keys, Opener, Sealer};
use nsh_roles::CapSet;
use nsh_wire::{Expect, Kind, SESSION_ID_LEN};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn start(&mut self, caps: CapSet, now_ms: u64, rand: Rand<'_>) -> Result<(), Why> {
        let login = self.admitted.take().ok_or(Why::Denied)?;
        let mut session_id = [0u8; SESSION_ID_LEN];
        if !rand(&mut session_id) {
            return Err(Why::Rand);
        }
        let names = caps.names();
        let names: Vec<&[u8]> = names.iter().map(|n| n.as_bytes()).collect();
        let body = auth_ok(&session_id, self.cfg.idle_s, &names).map_err(Why::Wire)?;
        let header = Header {
            session_id,
            start_ms: now_ms,
            th_server_hello: &self.th_hello,
            host_ed: self.signer.ed_public(),
            host_ml: self.signer.ml_public(),
            enrol: &self.cfg.enrol,
            doc_hash: &self.doc_hash,
            subject: &login.subject,
            user: &login.user,
            role: &login.role,
            grant_hash: &login.hash,
            caps: &names,
        };
        let record = Record::start(&header, self.cfg.max_record, self.cfg.ckpt_every);
        let mut record = record.map_err(Why::Wire)?;
        self.send(Kind::AuthOk, &body, Expect::AuthResult)?;
        self.th.push(Kind::AuthOk, &body);
        let hs = self.hs.take().ok_or(Why::Wire(nsh_wire::WireError::Order))?;
        let Keys { c2s, s2c } = session_keys(hs, &self.th.digest());
        (self.sealer, self.opener) = (Some(Sealer::new(s2c)), Some(Opener::new(c2s)));
        self.events.push(Event::Record(record.take_new()));
        self.events.push(Event::Started { session_id, caps });
        self.live = Some(Live {
            login,
            record,
            last_input_ms: now_ms,
            stopping: None,
            closing: false,
            pending: None,
            active: None,
        });
        self.stage = Stage::Session;
        Ok(())
    }
}
