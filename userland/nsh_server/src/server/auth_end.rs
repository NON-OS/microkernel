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

//! How a login ends without a session: AUTH_REFUSED with its one code,
//! or the client's ABORT, which is not answered.

use super::Server;
use crate::{Attestation, Event, HostSigner, Roles, Why};
use nsh_wire::{abort, Expect, Kind, Reason};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// `AUTH_REFUSED` with the code the refusal is told as, then the end.
    /// A trusted list missing is told `REVOCATION_STALE`, as a list too old.
    pub(super) fn refuse_login(&mut self, why: Why) -> Result<(), Why> {
        let reason = match why {
            Why::Login(r) => r.reason(),
            _ => Reason::RevocationStale,
        };
        if !reason.refuses_login() {
            return Err(why);
        }
        self.events.push(Event::Refused(why));
        self.send(Kind::AuthRefused, &reason.code().to_le_bytes(), Expect::AuthResult)?;
        self.end_quietly();
        Ok(())
    }

    /// The client's `ABORT`: the connection ends, and nothing is answered.
    pub(super) fn peer_abort(&mut self, body: &[u8]) -> Result<(), Why> {
        let reason = abort(body).map_err(Why::Wire)?;
        Err(Why::Aborted(reason))
    }
}
