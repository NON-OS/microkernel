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

//! What the caller sends once logged in: terminal input, the window size,
//! a signal, an escalation grant, and `CLOSE`. Each body is held to the
//! rules the server reads it by; a body it would refuse is not sent, and
//! the session goes on. Nothing can be sent before `AUTH_OK`.

use super::stage::Stage;
use super::Client;
use crate::Refusal;
use alloc::vec::Vec;
use nsh_wire::{escalate, Kind, Signal, Window, WireError, MAX_DATA};

impl<A, S> Client<A, S> {
    /// Terminal input, cut into frames of at most 16384 bytes.
    pub fn input(&mut self, bytes: &[u8]) -> Result<(), Refusal> {
        self.running()?;
        bytes.chunks(MAX_DATA).try_for_each(|c| self.send_session(Kind::Input, c))
    }

    pub fn window(&mut self, cols: u16, rows: u16) -> Result<(), Refusal> {
        self.running()?;
        let mut body = [0u8; 4];
        let n = Window { cols, rows }.encode(&mut body).map_err(Refusal::Unsendable)?;
        self.send_session(Kind::Window, &body[..n])
    }

    pub fn signal(&mut self, sig: Signal) -> Result<(), Refusal> {
        self.running()?;
        self.send_session(Kind::Signal, &[sig as u8])
    }

    /// Ask for an escalation with `grant`, an escalation grant. One at a
    /// time: the next waits for the answer to this one.
    pub fn escalate(&mut self, grant: &[u8]) -> Result<(), Refusal> {
        let mut p = self.running()?;
        let n = u16::try_from(grant.len()).map_err(|_| Refusal::Unsendable(WireError::Oversize))?;
        let mut body = Vec::with_capacity(2 + grant.len());
        body.extend_from_slice(&n.to_le_bytes());
        body.extend_from_slice(grant);
        escalate(&body).map_err(Refusal::Unsendable)?;
        if p.escalating {
            return Err(Refusal::NotNow);
        }
        self.send_session(Kind::Escalate, &body)?;
        p.escalating = true;
        self.stage = Stage::Session(p);
        Ok(())
    }
}
