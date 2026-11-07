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

//! What the host reads from the server: the bytes to send, the events,
//! and what identifies this connection.

use super::stage::Stage;
use super::Server;
use crate::{Attestation, HostSigner, Roles};
use alloc::vec::Vec;

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// The bytes to send to the client, in order. Each call hands over what
    /// has been queued since the last.
    pub fn take_output(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.out)
    }

    /// What has happened since the last call, in order.
    pub fn events(&mut self) -> Vec<crate::Event> {
        core::mem::take(&mut self.events)
    }

    /// The connection is over: the host closes it once the last output
    /// is sent.
    pub fn is_over(&self) -> bool {
        self.stage == Stage::Over
    }

    /// `TH(SERVER_HELLO)`, once the hello is sent.
    pub fn th_server_hello(&self) -> Option<[u8; 64]> {
        (self.stage != Stage::ClientHello).then_some(self.th_hello)
    }

    /// The session id, once `AUTH_OK` is sent.
    pub fn session_id(&self) -> Option<[u8; 16]> {
        self.live.as_ref().map(|l| l.record.session_id)
    }
}
