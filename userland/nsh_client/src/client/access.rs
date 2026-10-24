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

//! What the caller reads from the client: the bytes to send, the events,
//! and what identifies this session.

use super::stage::Stage;
use super::{Client, Event};
use alloc::vec::Vec;

impl<A, S> Client<A, S> {
    /// The bytes to send to the server, in order. Each call hands over what
    /// has been queued since the last.
    pub fn take_output(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.out)
    }

    /// What has happened since the last call, in order.
    pub fn events(&mut self) -> Vec<Event> {
        core::mem::take(&mut self.events)
    }

    /// The attestation checker the client was given.
    pub fn attester(&self) -> &A {
        &self.attester
    }

    /// The session is over: the caller closes the connection once it has
    /// sent the last output.
    pub fn is_over(&self) -> bool {
        matches!(self.stage, Stage::Over)
    }

    /// `TH(SERVER_HELLO)`, once the hello is in: the record names it.
    pub fn th_server_hello(&self) -> Option<[u8; 64]> {
        self.bound.transcript
    }

    /// The session id `AUTH_OK` gave, once logged in.
    pub fn session_id(&self) -> Option<[u8; 16]> {
        self.bound.session_id
    }
}
