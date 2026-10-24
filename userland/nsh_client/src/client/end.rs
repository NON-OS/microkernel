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

//! Ending a session. The server is told the one reason code a refusal
//! allows, and only once keys exist to seal it; then every key, the
//! X-Wing key pair and the handshake secret are dropped, which wipes them.

use super::stage::Stage;
use super::Client;
use crate::Refusal;
use nsh_wire::{Kind, Reason};

impl<A, S> Client<A, S> {
    /// End the session over `r`, telling the server what the wire allows.
    pub(super) fn refuse(&mut self, r: Refusal) -> Refusal {
        let told = match r {
            Refusal::Wire(e) => Some(e.reason()),
            Refusal::Host(_) => Some(Reason::Policy),
            Refusal::Record(_) => Some(Reason::Protocol),
            Refusal::RecordLimit => Some(Reason::Limit),
            _ => None,
        };
        if let Some(reason) = told.filter(|_| self.sealer.is_some()) {
            let peer = self.stage.peer();
            let _ = self.send(Kind::Abort, &reason.code().to_le_bytes(), peer);
        }
        self.end();
        r
    }

    /// End the session and drop every key, which wipes it.
    pub(super) fn end(&mut self) {
        self.stage = Stage::Over;
        (self.kem, self.hs, self.sealer, self.opener) = (None, None, None, None);
        nonos_hash::wipe(&mut self.record);
        self.record.clear();
    }
}
