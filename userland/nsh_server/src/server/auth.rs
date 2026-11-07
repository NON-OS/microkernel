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

//! `USER_AUTH`, checked by the role table in the order
//! docs/nsh/nsh-v1-roles.md gives: the proof of the key first, then the
//! grant, the revocation list, the role and the capabilities. A refusal is
//! answered `AUTH_REFUSED` with its one code and logged whole; a login
//! that passes is handed to the host to start.

use super::stage::Stage;
use super::Server;
use crate::{Admission, Attestation, Event, HostSigner, Roles, Why};
use nsh_roles::{login, Host};
use nsh_wire::{Kind, UserAuth, WireError};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn user_auth(&mut self, kind: Kind, body: &[u8], now_ms: u64) -> Result<(), Why> {
        match kind {
            Kind::UserAuth => {}
            Kind::Abort => return self.peer_abort(body),
            _ => return Err(Why::Wire(WireError::Order)),
        }
        let ua = UserAuth::parse(body).map_err(Why::Wire)?;
        let th = self.th.without_sigs(Kind::UserAuth, ua.signed);
        let checked = self.roles.with(|policy, list| match Host::new(policy, list, now_ms) {
            Ok(host) => login(&host, &ua, &th).map_err(Why::Login),
            Err(e) => Err(Why::Policy(e)),
        });
        let checked = checked.unwrap_or_else(|e| Err(Why::Policy(e)));
        let l = match checked {
            Ok(l) => l,
            Err(why) => return self.refuse_login(why),
        };
        self.th.push(Kind::UserAuth, body);
        self.events.push(Event::Admit(Admission {
            user: l.user.clone(),
            role: l.role.clone(),
            subject: l.subject,
            caps: l.caps,
            ends_ms: l.ends_ms,
        }));
        self.admitted = Some(l);
        self.stage = Stage::Admitting;
        Ok(())
    }
}
