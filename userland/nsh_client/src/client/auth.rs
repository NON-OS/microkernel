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

//! The answer to `USER_AUTH`. `AUTH_OK` closes the handshake: it goes into
//! the transcript, the session keys are made from the handshake secret
//! under `TH(AUTH_OK)`, and the handshake keys and secret are dropped,
//! which wipes them. `AUTH_REFUSED` ends the session; both sides close.

use super::stage::{Phase, Stage};
use super::{Client, Event, LoggedIn};
use crate::{Attester, Refusal, UserSigner};
use nsh_crypto::{session_keys, Keys, Opener, Sealer};
use nsh_wire::{abort, auth_refused, AuthOk, Kind, WireError};

impl<A: Attester, S: UserSigner> Client<A, S> {
    pub(super) fn auth_result(&mut self, kind: Kind, body: &[u8]) -> Result<(), Refusal> {
        let wire = Refusal::Wire;
        match kind {
            Kind::AuthOk => {}
            Kind::AuthRefused => {
                return Err(Refusal::AuthRefused(auth_refused(body).map_err(wire)?))
            }
            Kind::Abort => return Err(Refusal::Aborted(abort(body).map_err(wire)?)),
            _ => return Err(Refusal::Wire(WireError::Order)),
        }
        let ok = AuthOk::parse(body).map_err(wire)?;
        let hs = self.hs.take().ok_or(Refusal::Ended)?;
        self.bound.session_id = Some(*ok.session_id);
        self.th.push(Kind::AuthOk, body);
        let Keys { c2s, s2c } = session_keys(hs, &self.th.digest());
        self.sealer = Some(Sealer::new(c2s));
        self.opener = Some(Opener::new(s2c));
        self.stage = Stage::Session(Phase::START);
        self.events.push(Event::LoggedIn(LoggedIn {
            session_id: *ok.session_id,
            idle_s: ok.idle_s,
            caps: ok.caps.iter().map(<[u8]>::to_vec).collect(),
        }));
        Ok(())
    }
}
