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

//! `CLOSE` from the client, and what every send shares: the session must
//! be running, and a spent direction ends it.

use super::stage::{Ending, Phase, Stage};
use super::Client;
use crate::Refusal;
use nsh_wire::{Expect, Kind};

impl<A, S> Client<A, S> {
    /// Send `CLOSE`. The server answers with its record and its own
    /// `CLOSE`; the client sends nothing more.
    pub fn close(&mut self) -> Result<(), Refusal> {
        let Stage::Session(mut p) = self.stage else { return Err(Refusal::NotNow) };
        if p.closing {
            return Err(Refusal::NotNow);
        }
        self.send_session(Kind::Close, &[])?;
        p.closing = true;
        self.stage = Stage::Session(p);
        Ok(())
    }

    pub(super) fn running(&self) -> Result<Phase, Refusal> {
        match self.stage {
            Stage::Session(p) if p.ending == Ending::Running && !p.closing => Ok(p),
            _ => Err(Refusal::NotNow),
        }
    }

    /// A spent direction ends the session; a body refused locally does not.
    pub(super) fn send_session(&mut self, kind: Kind, body: &[u8]) -> Result<(), Refusal> {
        match self.send(kind, body, Expect::ServerSession) {
            Err(Refusal::Limit) => Err(self.refuse(Refusal::Limit)),
            other => other,
        }
    }
}
